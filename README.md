#  Bark 推送SDK

---

- 本SDK功能尚不完善，目前仅为个人使用，没有进行异常处理

##  普通消息推送

```rust

use anyhow::Result;
use bark_push::{BarkClient, BarkMessage, Level};

#[tokio::main]
async fn main() -> Result<()> {
  	// 1、新建client
    let client = BarkClient::new(
        "https://api.day.app",
        "YOUR_DEVICE_KEY",
        false,
        None,
        None,
    );
	// 2、构造message
    let msg = BarkMessage::new()
        .title("测试")
        .subtitle("推送设置副标题")
        .body("推送设置正文")
        .sound("alarm")
        .group("测试组1")
        .copy("www.google.com")
        .call("1")
        .volume(1)
        .level(Level::Critical);
		// 3、发送消息
    let res = client.send(&msg).await?;
    println!("code:{},msg:{}", res.code, res.message);

    Ok(())
}


```

##  加密消息推送

```rust
use anyhow::Result;
use bark_push::{BarkClient, BarkMessage, Level};

/// 加密消息推送
/// 目前仅支持 算法：AES128 模式：CBC Pading：pkcs7 （需要在app上配置，否则无法正确解密消息）
#[tokio::main]
async fn main() -> Result<()> {
    // 当is_encrypt 为 true 时，key 和 iv 必传
    let client = BarkClient::new(
        "https://api.day.app",
        "YOUR_DEVICE_KEY",
        true,
        Some("YOUR_KEY".to_owned()),
        Some("YOUR_IV".to_owned()),
    );

    let msg = BarkMessage::new()
        .title("测试")
        .subtitle("推送设置副标题")
        .body("推送设置正文")
        .sound("alarm")
        .group("测试组1")
        .copy("www.google.com")
        .call("1")
        .volume(1)
        .level(Level::Critical);

    let res = client.send(&msg).await?;
    println!("code:{},msg:{}", res.code, res.message);
    Ok(())
}

```

