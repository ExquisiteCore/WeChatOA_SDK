# WeChatOA SDK

微信公众号 API Rust SDK，提供完整的微信公众号开发接口封装。

## 功能特性

- **Access Token 自动管理** - 自动缓存和刷新，线程安全
- **消息收发** - 接收用户消息/事件，被动回复
- **客服消息** - 主动推送消息给用户
- **模板消息** - 发送模板消息
- **素材管理** - 临时/永久素材上传下载
- **用户管理** - 用户信息、列表、标签管理
- **自定义菜单** - 创建、查询、删除菜单
- **数据统计** - 用户分析、图文分析
- **草稿箱** - 草稿增删改查
- **文章发布** - 发布文章、群发消息
- **二维码** - 生成带参数二维码
- **签名验证** - 服务器回调签名校验

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
wechat-oa-sdk = "1.0"
tokio = { version = "1", features = ["full"] }
```

## 快速开始

```rust
use wechat_oa_sdk::{Config, WeChatClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let config = Config::new(
        "your_app_id",
        "your_app_secret",
        "your_token",  // 服务器配置中的 Token
    );
    let client = WeChatClient::new(config);

    // 获取 Access Token（自动缓存和刷新）
    let token = client.access_token().await?;
    println!("Token: {}", token);

    // 获取用户列表
    let users = client.get_user_list(None).await?;
    println!("粉丝数: {}", users.total);

    Ok(())
}
```

## 使用示例

### 发送客服消息

```rust
// 发送文本消息
client.send_text("用户OpenID", "Hello!").await?;

// 发送图片
client.send_image("用户OpenID", "media_id").await?;
```

### 发送模板消息

```rust
use std::collections::HashMap;
use wechat_oa_sdk::models::template::{TemplateMessage, TemplateDataItem};

let mut data = HashMap::new();
data.insert("first".to_string(), TemplateDataItem::new("您好"));
data.insert("keyword1".to_string(), TemplateDataItem::new("订单123"));

let msg = TemplateMessage::new("用户OpenID", "模板ID", data)
    .with_url("https://example.com");

client.send_template_message(&msg).await?;
```

### 素材管理

```rust
use wechat_oa_sdk::models::material::MaterialType;

// 上传临时素材
let data = std::fs::read("image.jpg")?;
let result = client.upload_temp_media(MaterialType::Image, "image.jpg", data).await?;
println!("Media ID: {}", result.media_id);

// 获取素材列表
let list = client.get_material_list(MaterialType::Image, 0, 20).await?;
```

### 发布文章

```rust
use wechat_oa_sdk::models::publish::Article;
use wechat_oa_sdk::models::material::MaterialType;

// 1. 上传封面图片
let cover_data = std::fs::read("cover.jpg")?;
let cover = client.upload_permanent_media(MaterialType::Image, "cover.jpg", cover_data).await?;

// 2. 创建文章
let article = Article::new("文章标题", "<p>文章内容HTML</p>", &cover.media_id)
    .with_author("作者")
    .with_digest("文章摘要");

// 3. 保存为草稿
let draft_id = client.add_draft(vec![article]).await?;

// 4. 发布（生成文章链接，不推送给粉丝）
let publish_id = client.submit_publish(&draft_id).await?;

// 或者群发（推送给所有粉丝）
// let result = client.mass_send_article(&draft_id, None, true).await?;
```

### 自定义菜单

```rust
use wechat_oa_sdk::models::menu::MenuButton;

let buttons = vec![
    MenuButton::click("点击按钮", "key1"),
    MenuButton::view("跳转链接", "https://example.com"),
    MenuButton::parent("更多", vec![
        MenuButton::click("子按钮1", "key2"),
        MenuButton::view("子按钮2", "https://example.com/page"),
    ]),
];

client.create_menu(buttons).await?;
```

### 用户管理

```rust
// 获取用户信息
let user = client.get_user_info("用户OpenID", None).await?;
println!("用户: {:?}", user);

// 创建标签
let tag = client.create_tag("VIP用户").await?;

// 给用户打标签
client.batch_tag_users(&["openid1", "openid2"], tag.id.unwrap()).await?;
```

### 生成二维码

```rust
use wechat_oa_sdk::models::qrcode::QrCodeAction;

// 创建临时二维码（30天有效）
let qr = client.create_qrcode(12345, QrCodeAction::Temporary, Some(2592000)).await?;

// 获取二维码图片 URL
let url = WeChatClient::get_qrcode_url(&qr.ticket);
println!("二维码链接: {}", url);

// 或者下载二维码图片
let image_data = client.download_qrcode(&qr.ticket).await?;
std::fs::write("qrcode.jpg", image_data)?;
```

### 处理消息回调

```rust
use wechat_oa_sdk::api::message::IncomingMessage;
use wechat_oa_sdk::models::reply::TextReply;

// 验证签名（GET 请求）
if client.verify_signature(signature, timestamp, nonce) {
    // 返回 echostr
}

// 解析消息（POST 请求）
let msg = client.parse_message(&xml_body)?;

match msg {
    IncomingMessage::Text(text) => {
        let reply = TextReply::new(
            &text.from_user_name,
            &text.to_user_name,
            "收到你的消息了！",
        );
        return Ok(reply.to_xml());
    }
    IncomingMessage::SubscribeEvent(event) => {
        let reply = TextReply::new(
            &event.from_user_name,
            &event.to_user_name,
            "欢迎关注！",
        );
        return Ok(reply.to_xml());
    }
    _ => {
        return Ok("success".to_string());
    }
}
```

## API 列表

### 基础
| 方法 | 说明 |
|------|------|
| `access_token()` | 获取 Access Token |
| `verify_signature()` | 验证消息签名 |
| `get_callback_ip_list()` | 获取微信服务器 IP |

### 消息
| 方法 | 说明 |
|------|------|
| `parse_message()` | 解析接收的消息 |
| `send_text()` | 发送文本客服消息 |
| `send_image()` | 发送图片客服消息 |
| `send_template_message()` | 发送模板消息 |

### 素材
| 方法 | 说明 |
|------|------|
| `upload_temp_media()` | 上传临时素材 |
| `get_temp_media()` | 获取临时素材 |
| `upload_permanent_media()` | 上传永久素材 |
| `get_material_count()` | 获取素材数量 |
| `get_material_list()` | 获取素材列表 |

### 用户
| 方法 | 说明 |
|------|------|
| `get_user_info()` | 获取用户信息 |
| `get_user_list()` | 获取用户列表 |
| `create_tag()` | 创建标签 |
| `batch_tag_users()` | 批量打标签 |

### 菜单
| 方法 | 说明 |
|------|------|
| `create_menu()` | 创建菜单 |
| `get_menu()` | 获取菜单 |
| `delete_menu()` | 删除菜单 |

### 草稿/发布
| 方法 | 说明 |
|------|------|
| `add_draft()` | 新建草稿 |
| `get_draft()` | 获取草稿 |
| `submit_publish()` | 发布文章 |
| `mass_send_article()` | 群发文章 |

### 二维码
| 方法 | 说明 |
|------|------|
| `create_qrcode()` | 创建二维码 |
| `download_qrcode()` | 下载二维码图片 |

## 运行测试

```bash
# 单元测试
cargo test

# 集成测试（需要真实账号）
WECHAT_APP_ID=xxx WECHAT_APP_SECRET=xxx WECHAT_TOKEN=xxx cargo run --example integration_test
```

## 注意事项

1. **IP 白名单**：调用 API 前需在公众号后台配置服务器 IP 白名单
2. **账号权限**：部分接口需要认证订阅号或服务号才能使用
3. **AppSecret 安全**：请勿泄露 AppSecret，建议使用环境变量管理

## License

MIT
