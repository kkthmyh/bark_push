use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Default)]
pub struct BarkMessage {
    /// 推送标题
    title: Option<String>,
    /// 推送副标题
    subtitle: Option<String>,
    /// 推送内容
    body: Option<String>,
    /// 设备 key（单推）
    device_key: Option<String>,
    /// 多设备 key 批量推送
    device_keys: Option<Vec<String>>,
    /// 推送中断级别："critical" / "active" / "timeSensitive" / "passive"
    level: Option<String>,
    /// 重要警告的通知音量，0-10
    volume: Option<u8>,
    /// 推送角标
    badge: Option<u32>,
    /// 传 "1" 通知铃声重复播放
    call: Option<String>,
    /// 传 "1" 自动复制推送内容
    auto_copy: Option<String>,
    /// 自定义复制内容，不传则复制全部推送内容
    copy: Option<String>,
    /// 推送铃声
    sound: Option<String>,
    /// 自定义图标 URL
    icon: Option<String>,
    /// 推送分组
    group: Option<String>,
    /// 加密推送密文
    ciphertext: Option<String>,
    /// "1" 保存推送，其他不保存
    is_archive: Option<String>,
    /// 点击跳转 URL
    url: Option<String>,
    /// "none" 时点击不弹窗
    action: Option<String>,
    /// 通知 ID，重复使用则覆盖
    id: Option<String>,
    /// "1" 删除通知（需搭配 id 使用）
    delete: Option<String>,
}

impl BarkMessage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self {
        self.subtitle = Some(v.into());
        self
    }
    pub fn body(mut self, v: impl Into<String>) -> Self {
        self.body = Some(v.into());
        self
    }
    pub fn device_key(mut self, v: impl Into<String>) -> Self {
        self.device_key = Some(v.into());
        self
    }
    pub fn device_keys(mut self, v: Vec<String>) -> Self {
        self.device_keys = Some(v);
        self
    }
    pub fn level(mut self, v: Level) -> Self {
        self.level = Some(v.as_str().to_string());
        self
    }
    pub fn volume(mut self, v: u8) -> Self {
        self.volume = Some(v);
        self
    }
    pub fn badge(mut self, v: u32) -> Self {
        self.badge = Some(v);
        self
    }
    pub fn call(mut self, v: impl Into<String>) -> Self {
        self.call = Some(v.into());
        self
    }
    pub fn auto_copy(mut self, v: impl Into<String>) -> Self {
        self.auto_copy = Some(v.into());
        self
    }
    pub fn copy(mut self, v: impl Into<String>) -> Self {
        self.copy = Some(v.into());
        self
    }
    pub fn sound(mut self, v: impl Into<String>) -> Self {
        self.sound = Some(v.into());
        self
    }
    pub fn icon(mut self, v: impl Into<String>) -> Self {
        self.icon = Some(v.into());
        self
    }
    pub fn group(mut self, v: impl Into<String>) -> Self {
        self.group = Some(v.into());
        self
    }
    pub fn ciphertext(mut self, v: impl Into<String>) -> Self {
        self.ciphertext = Some(v.into());
        self
    }
    pub fn is_archive(mut self, v: impl Into<String>) -> Self {
        self.is_archive = Some(v.into());
        self
    }
    pub fn url(mut self, v: impl Into<String>) -> Self {
        self.url = Some(v.into());
        self
    }
    pub fn action(mut self, v: impl Into<String>) -> Self {
        self.action = Some(v.into());
        self
    }
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    pub fn delete(mut self, v: impl Into<String>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    Critical,
    Active,
    TimeSensitive,
    Passive,
}

impl Level {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Level::Critical => "critical",
            Level::Active => "active",
            Level::TimeSensitive => "timeSensitive",
            Level::Passive => "passive",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BarkResponse {
    pub code: i32,
    pub message: String,
    pub timestamp: Option<i64>,
}
