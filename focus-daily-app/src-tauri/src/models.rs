use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 专注会话记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: i32,
    pub session_type: SessionType,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 会话类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Focus,     // 90分钟专注会话
    LongBreak, // 20分钟长休息
    MicroBreak, // 3-5分钟微休息
}

/// 创建新专注会话的输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFocusSession {
    pub session_type: SessionType,
    pub duration_seconds: i32,
}

/// 用户设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub id: String,
    pub theme: Theme,
    pub language: String,
    pub auto_start: bool,
    pub focus_duration_minutes: i32,
    pub long_break_duration_minutes: i32,
    pub micro_break_min_interval_minutes: i32,
    pub micro_break_max_interval_minutes: i32,
    pub micro_break_duration_seconds: i32,
    pub notifications_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 主题枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// 音频配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub id: String,
    pub config_type: AudioType,
    pub file_path: Option<String>,
    pub is_default: bool,
    pub volume: f32,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 音频类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioType {
    FocusStart,      // 专注开始
    FocusEnd,        // 专注结束
    LongBreakStart,  // 长休息开始
    LongBreakEnd,    // 长休息结束
    MicroBreakStart, // 微休息开始
}

/// 创建音频配置的输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAudioConfig {
    pub config_type: AudioType,
    pub file_path: Option<String>,
    pub volume: f32,
    pub enabled: bool,
}

/// 更新用户设置的输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserSettings {
    pub theme: Option<Theme>,
    pub language: Option<String>,
    pub auto_start: Option<bool>,
    pub focus_duration_minutes: Option<i32>,
    pub long_break_duration_minutes: Option<i32>,
    pub micro_break_min_interval_minutes: Option<i32>,
    pub micro_break_max_interval_minutes: Option<i32>,
    pub micro_break_duration_seconds: Option<i32>,
    pub notifications_enabled: Option<bool>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            theme: Theme::System,
            language: "zh-CN".to_string(),
            auto_start: false,
            focus_duration_minutes: 90,
            long_break_duration_minutes: 20,
            micro_break_min_interval_minutes: 3,
            micro_break_max_interval_minutes: 5,
            micro_break_duration_seconds: 15,
            notifications_enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
} 