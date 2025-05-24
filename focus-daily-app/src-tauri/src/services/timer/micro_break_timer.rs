use tokio::time::Instant;
use uuid::Uuid;
use rand::Rng;

use super::timer_state::TimerState;

/// 微休息定时器
pub struct MicroBreakTimer {
    /// 定时器状态
    state: TimerState,
    /// 开始时间
    start_instant: Option<Instant>,
}

impl MicroBreakTimer {
    /// 创建新的微休息定时器
    pub fn new(duration_seconds: u32) -> Self {
        Self {
            state: TimerState::new_micro_break_session(duration_seconds),
            start_instant: None,
        }
    }

    /// 开始微休息
    pub fn start(&mut self) -> String {
        let session_id = Uuid::new_v4().to_string();
        self.state.session_id = Some(session_id.clone());
        self.state.start();
        self.start_instant = Some(Instant::now());
        session_id
    }

    /// 跳过微休息
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
}

/// 微休息调度器
pub struct MicroBreakScheduler {
    /// 最小间隔（分钟）
    min_interval_minutes: u32,
    /// 最大间隔（分钟）
    max_interval_minutes: u32,
    /// 微休息持续时间（秒）
    duration_seconds: u32,
    /// 下次微休息时间（从专注开始计算的秒数）
    next_break_at: Option<u64>,
    /// 微休息计数
    break_count: u32,
}

impl MicroBreakScheduler {
    /// 创建新的微休息调度器
    pub fn new(min_interval_minutes: u32, max_interval_minutes: u32, duration_seconds: u32) -> Self {
        Self {
            min_interval_minutes,
            max_interval_minutes,
            duration_seconds,
            next_break_at: None,
            break_count: 0,
        }
    }

    /// 开始调度（专注会话开始时调用）
    pub fn start_scheduling(&mut self) {
        self.break_count = 0;
        self.schedule_next_break(0);
    }

    /// 检查是否应该触发微休息
    pub fn should_trigger_break(&self, elapsed_seconds: u64) -> bool {
        if let Some(next_break_at) = self.next_break_at {
            elapsed_seconds >= next_break_at
        } else {
            false
        }
    }

    /// 触发微休息后调用
    pub fn on_break_triggered(&mut self, current_elapsed: u64) {
        self.break_count += 1;
        // 计算下次微休息时间（当前时间 + 微休息时间 + 随机间隔）
        let next_schedule_from = current_elapsed + self.duration_seconds as u64;
        self.schedule_next_break(next_schedule_from);
    }

    /// 计算下次微休息时间
    fn schedule_next_break(&mut self, from_seconds: u64) {
        let min_interval = self.min_interval_minutes as u64 * 60;
        let max_interval = self.max_interval_minutes as u64 * 60;
        
        let mut rng = rand::thread_rng();
        let random_interval = rng.gen_range(min_interval..=max_interval);
        
        self.next_break_at = Some(from_seconds + random_interval);
    }

    /// 获取下次微休息时间
    pub fn next_break_at(&self) -> Option<u64> {
        self.next_break_at
    }

    /// 获取微休息计数
    pub fn break_count(&self) -> u32 {
        self.break_count
    }

    /// 重置调度器
    pub fn reset(&mut self) {
        self.next_break_at = None;
        self.break_count = 0;
    }
} 