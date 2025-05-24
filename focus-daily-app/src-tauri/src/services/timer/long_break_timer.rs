use uuid::Uuid;
use tokio::time::Instant;

use super::timer_state::TimerState;

/// 长休息定时器
pub struct LongBreakTimer {
    /// 定时器状态
    state: TimerState,
    /// 开始时间
    start_instant: Option<Instant>,
}

impl LongBreakTimer {
    /// 创建新的长休息定时器
    pub fn new(duration_minutes: u32) -> Self {
        Self {
            state: TimerState::new_long_break_session(duration_minutes),
            start_instant: None,
        }
    }

    /// 开始长休息
    pub fn start(&mut self) -> String {
        let session_id = Uuid::new_v4().to_string();
        self.state.session_id = Some(session_id.clone());
        self.state.start();
        self.start_instant = Some(Instant::now());
        session_id
    }

    /// 暂停长休息
    pub fn pause(&mut self) {
        self.state.pause();
    }

    /// 恢复长休息
    pub fn resume(&mut self) {
        self.state.resume();
    }

    /// 跳过长休息
    pub fn skip(&mut self) {
        self.state.complete();
    }

    /// 重置定时器
    pub fn reset(&mut self) {
        self.state.reset();
        self.start_instant = None;
    }

    /// 获取当前状态
    pub fn get_state(&self) -> &TimerState {
        &self.state
    }

    /// 更新定时器（每秒调用）
    pub fn tick(&mut self) -> bool {
        if !self.state.is_running() {
            return false;
        }

        if self.state.remaining_duration > 0 {
            self.state.remaining_duration -= 1;
            self.state.elapsed_duration += 1;
            true
        } else {
            self.state.complete();
            false
        }
    }

    /// 检查是否完成
    pub fn is_completed(&self) -> bool {
        self.state.is_completed()
    }

    /// 获取剩余时间（秒）
    pub fn remaining_seconds(&self) -> u64 {
        self.state.remaining_duration
    }

    /// 获取进度百分比
    pub fn progress(&self) -> f64 {
        self.state.progress()
    }

    /// 获取已用时间（秒）
    pub fn elapsed_seconds(&self) -> u64 {
        self.state.elapsed_duration
    }

    /// 获取总时长（秒）
    pub fn total_seconds(&self) -> u64 {
        self.state.total_duration
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        self.state.is_running()
    }

    /// 检查是否已暂停
    pub fn is_paused(&self) -> bool {
        self.state.is_paused()
    }
} 