use anyhow::Result;
use bark_push::{BarkClient, BarkMessage, Level};

#[tokio::main]
async fn main() -> Result<()> {
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
