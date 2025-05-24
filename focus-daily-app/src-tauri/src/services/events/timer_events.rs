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
            TimerEvent::PhaseChanged { .. } => {
                self.app_handle.emit("phase-changed", &event)?;
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
}

/// 定时器事件监听器特征
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
    pub const PHASE_CHANGED: &str = "phase-changed";
    pub const TIMER_EVENT: &str = "timer-event";
    pub const TIMER_STATE_UPDATE: &str = "timer-state-update";
    pub const TIMER_ERROR: &str = "timer-error";
} 