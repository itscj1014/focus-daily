use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 定时器状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimerStatus {
    Idle,       // 空闲状态
    Running,    // 运行中
    Paused,     // 暂停
    Completed,  // 已完成
}

/// 会话阶段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionPhase {
    Focus,          // 专注阶段
    LongBreak,      // 长休息阶段
    MicroBreak,     // 微休息阶段
}

/// 定时器状态数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    /// 当前状态
    pub status: TimerStatus,
    /// 当前阶段
    pub phase: SessionPhase,
    /// 总时长（秒）
    pub total_duration: u64,
    /// 剩余时长（秒）
    pub remaining_duration: u64,
    /// 已经过时长（秒）
    pub elapsed_duration: u64,
    /// 开始时间
    pub start_time: Option<DateTime<Utc>>,
    /// 暂停时间
    pub pause_time: Option<DateTime<Utc>>,
    /// 会话ID
    pub session_id: Option<String>,
    /// 微休息计数器
    pub micro_break_count: u32,
    /// 下次微休息时间（秒）
    pub next_micro_break_at: Option<u64>,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            status: TimerStatus::Idle,
            phase: SessionPhase::Focus,
            total_duration: 0,
            remaining_duration: 0,
            elapsed_duration: 0,
            start_time: None,
            pause_time: None,
            session_id: None,
            micro_break_count: 0,
            next_micro_break_at: None,
        }
    }
}

impl TimerState {
    /// 创建新的专注会话状态
    pub fn new_focus_session(duration_minutes: u32) -> Self {
        let total_seconds = (duration_minutes * 60) as u64;
        Self {
            status: TimerStatus::Idle,
            phase: SessionPhase::Focus,
            total_duration: total_seconds,
            remaining_duration: total_seconds,
            elapsed_duration: 0,
            start_time: None,
            pause_time: None,
            session_id: None,
            micro_break_count: 0,
            next_micro_break_at: None,
        }
    }

    /// 创建新的长休息状态
    pub fn new_long_break_session(duration_minutes: u32) -> Self {
        let total_seconds = (duration_minutes * 60) as u64;
        Self {
            status: TimerStatus::Idle,
            phase: SessionPhase::LongBreak,
            total_duration: total_seconds,
            remaining_duration: total_seconds,
            elapsed_duration: 0,
            start_time: None,
            pause_time: None,
            session_id: None,
            micro_break_count: 0,
            next_micro_break_at: None,
        }
    }

    /// 创建新的微休息状态
    pub fn new_micro_break_session(duration_seconds: u32) -> Self {
        let total_seconds = duration_seconds as u64;
        Self {
            status: TimerStatus::Idle,
            phase: SessionPhase::MicroBreak,
            total_duration: total_seconds,
            remaining_duration: total_seconds,
            elapsed_duration: 0,
            start_time: None,
            pause_time: None,
            session_id: None,
            micro_break_count: 0,
            next_micro_break_at: None,
        }
    }

    /// 开始定时器
    pub fn start(&mut self) {
        self.status = TimerStatus::Running;
        self.start_time = Some(Utc::now());
        self.pause_time = None;
    }

    /// 暂停定时器
    pub fn pause(&mut self) {
        if self.status == TimerStatus::Running {
            self.status = TimerStatus::Paused;
            self.pause_time = Some(Utc::now());
        }
    }

    /// 恢复定时器
    pub fn resume(&mut self) {
        if self.status == TimerStatus::Paused {
            self.status = TimerStatus::Running;
            self.pause_time = None;
        }
    }

    /// 完成定时器
    pub fn complete(&mut self) {
        self.status = TimerStatus::Completed;
        self.remaining_duration = 0;
        self.elapsed_duration = self.total_duration;
    }

    /// 重置定时器
    pub fn reset(&mut self) {
        self.status = TimerStatus::Idle;
        self.remaining_duration = self.total_duration;
        self.elapsed_duration = 0;
        self.start_time = None;
        self.pause_time = None;
    }

    /// 更新剩余时间
    pub fn update_remaining(&mut self, remaining: u64) {
        self.remaining_duration = remaining;
        self.elapsed_duration = self.total_duration.saturating_sub(remaining);
    }

    /// 获取进度百分比 (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_duration == 0 {
            return 0.0;
        }
        self.elapsed_duration as f64 / self.total_duration as f64
    }

    /// 检查是否已完成
    pub fn is_completed(&self) -> bool {
        self.status == TimerStatus::Completed || self.remaining_duration == 0
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        self.status == TimerStatus::Running
    }

    /// 检查是否已暂停
    pub fn is_paused(&self) -> bool {
        self.status == TimerStatus::Paused
    }

    /// 检查是否空闲
    pub fn is_idle(&self) -> bool {
        self.status == TimerStatus::Idle
    }
}

/// 定时器事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimerEvent {
    /// 定时器开始
    Started {
        phase: SessionPhase,
        duration: u64,
        session_id: String,
    },
    /// 定时器暂停
    Paused {
        phase: SessionPhase,
        remaining: u64,
    },
    /// 定时器恢复
    Resumed {
        phase: SessionPhase,
        remaining: u64,
    },
    /// 定时器更新（每秒触发）
    Tick {
        phase: SessionPhase,
        remaining: u64,
        elapsed: u64,
        progress: f64,
    },
    /// 定时器完成
    Completed {
        phase: SessionPhase,
        session_id: String,
    },
    /// 定时器重置
    Reset {
        phase: SessionPhase,
    },
    /// 微休息触发
    MicroBreakTriggered {
        count: u32,
        duration: u64,
    },
    /// 阶段切换
    PhaseChanged {
        from: SessionPhase,
        to: SessionPhase,
    },
} 