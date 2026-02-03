use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{
    Engine, alphabet,
    engine::{GeneralPurpose, GeneralPurposeConfig, general_purpose::STANDARD as BASE64},
};
use rand::Rng;
use sha1::{Digest, Sha1};

use crate::error::{Result, WeChatError};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

const AES_BLOCK_SIZE: usize = 16;

/// 宽松 Base64 解码器（允许尾部 bits 不为 0）
///
/// 微信生成的 EncodingAESKey 可能在 Base64 尾部填充位不为 0，
/// 标准解码器会拒绝，这里使用宽松模式兼容。
const LENIENT_BASE64: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
);

/// Verify the WeChat server callback signature.
///
/// WeChat sends `signature`, `timestamp`, and `nonce` as query parameters.
/// The verification algorithm:
/// 1. Sort `[token, timestamp, nonce]` lexicographically
/// 2. Concatenate them
/// 3. SHA1 hash the result
/// 4. Compare with the provided signature
pub fn check_signature(token: &str, signature: &str, timestamp: &str, nonce: &str) -> bool {
    let computed = compute_signature(token, timestamp, nonce);
    computed == signature
}

fn compute_signature(token: &str, timestamp: &str, nonce: &str) -> String {
    let mut parts = [token, timestamp, nonce];
    parts.sort();
    let input = parts.join("");

    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Compute message signature for encrypted messages.
///
/// Used to verify incoming encrypted messages and sign outgoing encrypted messages.
/// Algorithm: SHA1(sort([token, timestamp, nonce, encrypt_msg]))
pub fn compute_msg_signature(
    token: &str,
    timestamp: &str,
    nonce: &str,
    encrypt_msg: &str,
) -> String {
    let mut parts = [token, timestamp, nonce, encrypt_msg];
    parts.sort();
    let input = parts.join("");

    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Verify the encrypted message signature.
pub fn check_msg_signature(
    token: &str,
    msg_signature: &str,
    timestamp: &str,
    nonce: &str,
    encrypt_msg: &str,
) -> bool {
    let computed = compute_msg_signature(token, timestamp, nonce, encrypt_msg);
    computed == msg_signature
}

/// Decode the EncodingAESKey from WeChat settings.
///
/// EncodingAESKey is a 43-character Base64-encoded string.
/// We need to append "=" to make it valid Base64, then decode to get 32 bytes.
///
/// Note: WeChat's EncodingAESKey may have non-zero trailing bits, so we use
/// a lenient decoder that allows this.
pub fn decode_aes_key(encoding_aes_key: &str) -> Result<[u8; 32]> {
    if encoding_aes_key.len() != 43 {
        return Err(WeChatError::InvalidAesKey);
    }

    let padded = format!("{}=", encoding_aes_key);
    // 使用宽松解码器，兼容微信可能生成的非标准 Base64
    let decoded = LENIENT_BASE64
        .decode(padded)
        .map_err(|_| WeChatError::InvalidAesKey)?;

    if decoded.len() != 32 {
        return Err(WeChatError::InvalidAesKey);
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&decoded);
    Ok(key)
}

/// Decrypt an encrypted message from WeChat.
///
/// Returns (decrypted_xml, app_id).
///
/// The encrypted message format after Base64 decoding:
/// - 16 bytes: random data
/// - 4 bytes: message length (big-endian)
/// - N bytes: message content (XML)
/// - M bytes: AppID
pub fn decrypt_message(aes_key: &[u8; 32], encrypted: &str) -> Result<(String, String)> {
    // Base64 decode
    let ciphertext = BASE64
        .decode(encrypted)
        .map_err(|e| WeChatError::DecryptionFailed(format!("Base64 decode failed: {}", e)))?;

    if ciphertext.len() < AES_BLOCK_SIZE || ciphertext.len() % AES_BLOCK_SIZE != 0 {
        return Err(WeChatError::DecryptionFailed(
            "Invalid ciphertext length".to_string(),
        ));
    }

    // IV is the first 16 bytes of the AES key
    let iv: [u8; 16] = aes_key[..16].try_into().unwrap();

    // Decrypt using AES-256-CBC
    let mut buf = ciphertext.to_vec();
    let decrypted = Aes256CbcDec::new(aes_key.into(), &iv.into())
        .decrypt_padded_mut::<aes::cipher::block_padding::NoPadding>(&mut buf)
        .map_err(|e| WeChatError::DecryptionFailed(format!("AES decrypt failed: {}", e)))?;

    // Remove PKCS#7 padding
    let unpadded = pkcs7_unpad(decrypted)?;

    // Parse the decrypted content
    // Format: random(16) + msg_len(4) + msg(N) + app_id(M)
    if unpadded.len() < 20 {
        return Err(WeChatError::InvalidMessageFormat);
    }

    // Skip 16 bytes random data
    let content = &unpadded[16..];

    // Read message length (big-endian)
    let msg_len = u32::from_be_bytes([content[0], content[1], content[2], content[3]]) as usize;

    if content.len() < 4 + msg_len {
        return Err(WeChatError::InvalidMessageFormat);
    }

    // Extract message and AppID
    let msg = &content[4..4 + msg_len];
    let app_id = &content[4 + msg_len..];

    let xml = String::from_utf8(msg.to_vec())
        .map_err(|e| WeChatError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))?;
    let app_id = String::from_utf8(app_id.to_vec())
        .map_err(|e| WeChatError::DecryptionFailed(format!("Invalid UTF-8 in AppID: {}", e)))?;

    Ok((xml, app_id))
}

/// Encrypt a message for WeChat.
///
/// The plaintext format:
/// - 16 bytes: random data
/// - 4 bytes: message length (big-endian)
/// - N bytes: message content (XML)
/// - M bytes: AppID
pub fn encrypt_message(aes_key: &[u8; 32], app_id: &str, xml: &str) -> Result<String> {
    let xml_bytes = xml.as_bytes();
    let app_id_bytes = app_id.as_bytes();

    // Generate 16 random bytes
    let random_bytes: [u8; 16] = rand::random();

    // Build plaintext: random(16) + msg_len(4) + msg + app_id
    let msg_len = xml_bytes.len() as u32;
    let msg_len_bytes = msg_len.to_be_bytes();

    let mut plaintext = Vec::with_capacity(16 + 4 + xml_bytes.len() + app_id_bytes.len());
    plaintext.extend_from_slice(&random_bytes);
    plaintext.extend_from_slice(&msg_len_bytes);
    plaintext.extend_from_slice(xml_bytes);
    plaintext.extend_from_slice(app_id_bytes);

    // Apply PKCS#7 padding
    let padded = pkcs7_pad(&plaintext, AES_BLOCK_SIZE);

    // IV is the first 16 bytes of the AES key
    let iv: [u8; 16] = aes_key[..16].try_into().unwrap();

    // Encrypt using AES-256-CBC
    let mut buf = padded;
    let buf_len = buf.len();
    let ciphertext = Aes256CbcEnc::new(aes_key.into(), &iv.into())
        .encrypt_padded_mut::<aes::cipher::block_padding::NoPadding>(&mut buf, buf_len)
        .map_err(|e| WeChatError::EncryptionFailed(format!("AES encrypt failed: {}", e)))?;

    // Base64 encode
    Ok(BASE64.encode(ciphertext))
}

/// Apply PKCS#7 padding.
fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded = data.to_vec();
    padded.extend(std::iter::repeat(padding_len as u8).take(padding_len));
    padded
}

/// Remove PKCS#7 padding.
fn pkcs7_unpad(data: &[u8]) -> Result<&[u8]> {
    if data.is_empty() {
        return Err(WeChatError::DecryptionFailed("Empty data".to_string()));
    }

    let padding_len = data[data.len() - 1] as usize;

    if padding_len == 0 || padding_len > AES_BLOCK_SIZE || padding_len > data.len() {
        return Err(WeChatError::DecryptionFailed("Invalid padding".to_string()));
    }

    // Verify all padding bytes
    for &byte in &data[data.len() - padding_len..] {
        if byte as usize != padding_len {
            return Err(WeChatError::DecryptionFailed("Invalid padding".to_string()));
        }
    }

    Ok(&data[..data.len() - padding_len])
}

/// Generate a random nonce string.
pub fn generate_nonce() -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    (0..16)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Generate the encrypted reply XML.
pub fn generate_encrypted_xml(
    encrypt_msg: &str,
    msg_signature: &str,
    timestamp: &str,
    nonce: &str,
) -> String {
    format!(
        r#"<xml>
<Encrypt><![CDATA[{}]]></Encrypt>
<MsgSignature><![CDATA[{}]]></MsgSignature>
<TimeStamp>{}</TimeStamp>
<Nonce><![CDATA[{}]]></Nonce>
</xml>"#,
        encrypt_msg, msg_signature, timestamp, nonce
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_signature() {
        let token = "test_token";
        let timestamp = "1234567890";
        let nonce = "abc123";
        let signature = compute_signature(token, timestamp, nonce);
        assert!(check_signature(token, &signature, timestamp, nonce));
        assert!(!check_signature(token, "wrong_signature", timestamp, nonce));
    }

    #[test]
    fn test_decode_aes_key() {
        // Valid 43-char key (Base64 encoded 32 bytes without trailing =)
        // This is "0123456789012345678901234567890A" in Base64 (without =)
        let key = "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MEE";
        let result = decode_aes_key(key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);

        // Invalid length
        let short_key = "abc";
        assert!(decode_aes_key(short_key).is_err());
    }

    #[test]
    fn test_pkcs7_pad_unpad() {
        let data = b"hello";
        let padded = pkcs7_pad(data, 16);
        assert_eq!(padded.len(), 16);
        assert_eq!(padded[5..], [11u8; 11]);

        let unpadded = pkcs7_unpad(&padded).unwrap();
        assert_eq!(unpadded, data);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // Valid 43-char key (Base64 encoded 32 bytes without trailing =)
        let key_str = "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MEE";
        let aes_key = decode_aes_key(key_str).unwrap();
        let app_id = "wx1234567890";
        let xml = "<xml><Content>Hello</Content></xml>";

        let encrypted = encrypt_message(&aes_key, app_id, xml).unwrap();
        let (decrypted_xml, decrypted_app_id) = decrypt_message(&aes_key, &encrypted).unwrap();

        assert_eq!(decrypted_xml, xml);
        assert_eq!(decrypted_app_id, app_id);
    }

    #[test]
    fn test_msg_signature() {
        let token = "test_token";
        let timestamp = "1234567890";
        let nonce = "abc123";
        let encrypt_msg = "encrypted_content";

        let sig = compute_msg_signature(token, timestamp, nonce, encrypt_msg);
        assert!(check_msg_signature(
            token,
            &sig,
            timestamp,
            nonce,
            encrypt_msg
        ));
        assert!(!check_msg_signature(
            token,
            "wrong",
            timestamp,
            nonce,
            encrypt_msg
        ));
    }

    #[test]
    fn test_generate_nonce() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        assert_eq!(nonce1.len(), 16);
        assert_eq!(nonce2.len(), 16);
        assert_ne!(nonce1, nonce2);
    }
}
