use tauri::{AppHandle, Emitter};

use crate::services::timer::timer_state::TimerEvent;

/// 定时器事件发送器
pub struct TimerEventEmitter {
    app_handle: AppHandle,
}

impl TimerEventEmitter {
    /// 创建新的事件发送器
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    /// 发送定时器事件
    pub async fn emit_timer_event(&self, event: TimerEvent) -> Result<(), tauri::Error> {
        match &event {
            TimerEvent::Started { .. } => {
                self.app_handle.emit("timer-started", &event)?;
            }
            TimerEvent::Paused { .. } => {
                self.app_handle.emit("timer-paused", &event)?;
            }
            TimerEvent::Resumed { .. } => {
                self.app_handle.emit("timer-resumed", &event)?;
            }
            TimerEvent::Tick { .. } => {
                self.app_handle.emit("timer-tick", &event)?;
            }
            TimerEvent::Completed { .. } => {
                self.app_handle.emit("timer-completed", &event)?;
            }
            TimerEvent::Reset { .. } => {
                self.app_handle.emit("timer-reset", &event)?;
            }
            TimerEvent::MicroBreakTriggered { .. } => {
                self.app_handle.emit("micro-break-triggered", &event)?;
            }
            TimerEvent::MicroBreakSkipped { .. } => {
                self.app_handle.emit("micro-break-skipped", &event)?;
            }
            TimerEvent::MicroBreakCompleted { .. } => {
                self.app_handle.emit("micro-break-completed", &event)?;
            }
            TimerEvent::MicroBreakSkipLimitReached { .. } => {
                self.app_handle.emit("micro-break-skip-limit-reached", &event)?;
            }
            TimerEvent::MicroBreakScheduleUpdated { .. } => {
                self.app_handle.emit("micro-break-schedule-updated", &event)?;
            }
            TimerEvent::MicroBreakStatsUpdated { .. } => {
                self.app_handle.emit("micro-break-stats-updated", &event)?;
            }
            TimerEvent::PhaseChanged { .. } => {
                self.app_handle.emit("phase-changed", &event)?;
            }
            TimerEvent::FatigueWarning { .. } => {
                self.app_handle.emit("fatigue-warning", &event)?;
            }
            TimerEvent::EfficiencyFeedback { .. } => {
                self.app_handle.emit("efficiency-feedback", &event)?;
            }
        }

        // 同时发送通用的timer-event事件
        self.app_handle.emit("timer-event", &event)?;
        
        Ok(())
    }

    /// 发送状态更新事件
    pub async fn emit_state_update(&self, state: &crate::services::timer::TimerState) -> Result<(), tauri::Error> {
        self.app_handle.emit("timer-state-update", state)?;
        Ok(())
    }

    /// 发送错误事件
    pub async fn emit_error(&self, error: &str) -> Result<(), tauri::Error> {
        let error_data = serde_json::json!({
            "error": error,
            "timestamp": chrono::Utc::now()
        });
        self.app_handle.emit("timer-error", error_data)?;
        Ok(())
    }

    /// 发送微休息通知
    pub async fn emit_micro_break_notification(&self, message: &str, notification_type: &str) -> Result<(), tauri::Error> {
        let notification_data = serde_json::json!({
            "message": message,
            "type": notification_type,
            "timestamp": chrono::Utc::now()
        });
        self.app_handle.emit("micro-break-notification", notification_data)?;
        Ok(())
    }

    /// 发送疲劳度预警通知
    pub async fn emit_fatigue_alert(&self, level: f32, recommendation: &str) -> Result<(), tauri::Error> {
        let alert_data = serde_json::json!({
            "fatigue_level": level,
            "recommendation": recommendation,
            "timestamp": chrono::Utc::now(),
            "priority": if level > 0.8 { "high" } else if level > 0.5 { "medium" } else { "low" }
        });
        self.app_handle.emit("fatigue-alert", alert_data)?;
        Ok(())
    }
}

/// 增强的定时器事件监听器特征
pub trait EnhancedTimerEventListener {
    /// 处理定时器开始事件
    fn on_timer_started(&self, phase: &str, duration: u64, session_id: &str);
    
    /// 处理定时器暂停事件
    fn on_timer_paused(&self, phase: &str, remaining: u64);
    
    /// 处理定时器恢复事件
    fn on_timer_resumed(&self, phase: &str, remaining: u64);
    
    /// 处理定时器tick事件
    fn on_timer_tick(&self, phase: &str, remaining: u64, elapsed: u64, progress: f64);
    
    /// 处理定时器完成事件
    fn on_timer_completed(&self, phase: &str, session_id: &str);
    
    /// 处理定时器重置事件
    fn on_timer_reset(&self, phase: &str);
    
    /// 处理微休息触发事件
    fn on_micro_break_triggered(&self, count: u32, duration: u64);
    
    /// 处理微休息跳过事件
    fn on_micro_break_skipped(&self, count: u32, remaining_skips: u32, skip_limit: u32);
    
    /// 处理微休息完成事件
    fn on_micro_break_completed(&self, count: u32, completion_rate: f32);
    
    /// 处理微休息跳过限制达到事件
    fn on_micro_break_skip_limit_reached(&self, skip_limit: u32);
    
    /// 处理微休息调度更新事件
    fn on_micro_break_schedule_updated(&self, next_break_at: Option<u64>, adjustment_factor: f32, fatigue_level: f32);
    
    /// 处理微休息统计更新事件
    fn on_micro_break_stats_updated(&self, total: u32, completed: u32, skipped: u32, completion_rate: f32);
    
    /// 处理阶段切换事件
    fn on_phase_changed(&self, from: &str, to: &str);
    
    /// 处理疲劳警告事件
    fn on_fatigue_warning(&self, level: f32, recommendation: &str);
    
    /// 处理效率反馈事件
    fn on_efficiency_feedback(&self, session_id: &str, focus_quality: f32, micro_break_effectiveness: f32);
}

/// 定时器事件监听器特征（保持向后兼容）
pub trait TimerEventListener {
    /// 处理定时器开始事件
    fn on_timer_started(&self, phase: &str, duration: u64, session_id: &str);
    
    /// 处理定时器暂停事件
    fn on_timer_paused(&self, phase: &str, remaining: u64);
    
    /// 处理定时器恢复事件
    fn on_timer_resumed(&self, phase: &str, remaining: u64);
    
    /// 处理定时器tick事件
    fn on_timer_tick(&self, phase: &str, remaining: u64, elapsed: u64, progress: f64);
    
    /// 处理定时器完成事件
    fn on_timer_completed(&self, phase: &str, session_id: &str);
    
    /// 处理定时器重置事件
    fn on_timer_reset(&self, phase: &str);
    
    /// 处理微休息触发事件
    fn on_micro_break_triggered(&self, count: u32, duration: u64);
    
    /// 处理阶段切换事件
    fn on_phase_changed(&self, from: &str, to: &str);
}

/// 事件常量
pub mod event_names {
    pub const TIMER_STARTED: &str = "timer-started";
    pub const TIMER_PAUSED: &str = "timer-paused";
    pub const TIMER_RESUMED: &str = "timer-resumed";
    pub const TIMER_TICK: &str = "timer-tick";
    pub const TIMER_COMPLETED: &str = "timer-completed";
    pub const TIMER_RESET: &str = "timer-reset";
    pub const MICRO_BREAK_TRIGGERED: &str = "micro-break-triggered";
    pub const MICRO_BREAK_SKIPPED: &str = "micro-break-skipped";
    pub const MICRO_BREAK_COMPLETED: &str = "micro-break-completed";
    pub const MICRO_BREAK_SKIP_LIMIT_REACHED: &str = "micro-break-skip-limit-reached";
    pub const MICRO_BREAK_SCHEDULE_UPDATED: &str = "micro-break-schedule-updated";
    pub const MICRO_BREAK_STATS_UPDATED: &str = "micro-break-stats-updated";
    pub const PHASE_CHANGED: &str = "phase-changed";
    pub const FATIGUE_WARNING: &str = "fatigue-warning";
    pub const EFFICIENCY_FEEDBACK: &str = "efficiency-feedback";
    pub const TIMER_EVENT: &str = "timer-event";
    pub const TIMER_STATE_UPDATE: &str = "timer-state-update";
    pub const TIMER_ERROR: &str = "timer-error";
    pub const MICRO_BREAK_NOTIFICATION: &str = "micro-break-notification";
    pub const FATIGUE_ALERT: &str = "fatigue-alert";
} 