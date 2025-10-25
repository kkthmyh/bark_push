use crate::message::{BarkMessage, BarkResponse};
use anyhow::{Ok, Result};
use base64::{Engine as _, engine::general_purpose};
use openssl::symm::{Cipher, encrypt};
use reqwest::Client;

pub struct BarkClient {
    client: Client,
    pub(crate) base_url: String,
    pub(crate) device_key: String,
    pub(crate) is_encrypt: bool,
    pub(crate) key: Option<String>,
    pub(crate) iv: Option<String>,
}

impl BarkClient {
    pub fn new(
        base_url: &str,
        device_key: &str,
        is_encrypt: bool,
        key: Option<String>,
        iv: Option<String>,
    ) -> Self {
        BarkClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            device_key: device_key.to_string(),
            is_encrypt,
            key,
            iv,
        }
    }

    pub async fn send(&self, msg: &BarkMessage) -> Result<BarkResponse> {
        let mut url = self.base_url.clone();
        url.push_str("/");
        url.push_str(&self.device_key);
        match &self.is_encrypt {
            true => {
                let message = serde_json::to_string(&msg)?;
                let ciphertext = encrypt(
                    Cipher::aes_128_cbc(),
                    self.key.clone().unwrap().as_bytes(),
                    Some(self.iv.clone().unwrap().as_bytes()),
                    message.as_bytes(),
                )?;
                let ciphertext_base64 = general_purpose::STANDARD.encode(&ciphertext);
                let res = self
                    .client
                    .post(url)
                    .form(&[
                        ("ciphertext", ciphertext_base64),
                        ("iv", self.iv.clone().unwrap()),
                    ])
                    .send()
                    .await?;
                let bark_response: BarkResponse = res.json().await?;
                Ok(bark_response)
            }
            false => {
                let res = self.client.post(url).json(&msg).send().await?;
                let bark_response: BarkResponse = res.json().await?;
                Ok(bark_response)
            }
        }
    }
}
