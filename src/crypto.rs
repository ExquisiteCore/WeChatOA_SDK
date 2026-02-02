use sha1::{Digest, Sha1};

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
}
