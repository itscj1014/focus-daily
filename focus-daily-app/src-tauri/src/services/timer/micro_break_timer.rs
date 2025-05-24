use tokio::time::Instant;
use uuid::Uuid;
use rand::Rng;
use chrono::{DateTime, Utc};

use super::timer_state::TimerState;

/// 微休息定时器
pub struct MicroBreakTimer {
    /// 定时器状态
    state: TimerState,
    /// 开始时间
    start_instant: Option<Instant>,
    /// 跳过次数限制
    skip_limit: u32,
    /// 当前会话中已跳过次数
    skipped_count: u32,
}

impl MicroBreakTimer {
    /// 创建新的微休息定时器
    pub fn new(duration_seconds: u32) -> Self {
        Self {
            state: TimerState::new_micro_break_session(duration_seconds),
            start_instant: None,
            skip_limit: 3, // 默认每个专注会话最多跳过3次微休息
            skipped_count: 0,
        }
    }

    /// 创建带跳过限制的微休息定时器
    pub fn new_with_skip_limit(duration_seconds: u32, skip_limit: u32) -> Self {
        Self {
            state: TimerState::new_micro_break_session(duration_seconds),
            start_instant: None,
            skip_limit,
            skipped_count: 0,
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
    pub fn skip(&mut self) -> Result<(), String> {
        if self.skipped_count >= self.skip_limit {
            return Err(format!("已达到跳过限制（{}/{}），请完成此次微休息", 
                self.skipped_count, self.skip_limit));
        }
        
        self.skipped_count += 1;
        self.state.complete();
        Ok(())
    }

    /// 检查是否可以跳过
    pub fn can_skip(&self) -> bool {
        self.skipped_count < self.skip_limit
    }

    /// 获取剩余跳过次数
    pub fn remaining_skips(&self) -> u32 {
        self.skip_limit.saturating_sub(self.skipped_count)
    }

    /// 重置定时器和跳过计数
    pub fn reset(&mut self) {
        self.state.reset();
        self.start_instant = None;
        self.skipped_count = 0;
    }

    /// 重置跳过计数（新的专注会话开始时调用）
    pub fn reset_skip_count(&mut self) {
        self.skipped_count = 0;
    }

    /// 获取当前状态
    pub fn get_state(&self) -> &TimerState {
        &self.state
    }

    /// 获取跳过统计
    pub fn get_skip_stats(&self) -> (u32, u32) {
        (self.skipped_count, self.skip_limit)
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

/// 智能微休息调度器
pub struct EnhancedMicroBreakScheduler {
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
    /// 跳过计数
    skip_count: u32,
    /// 完成计数
    completed_count: u32,
    /// 历史间隔调整因子（根据用户行为自适应）
    interval_adjustment_factor: f32,
    /// 最后一次微休息时间
    last_break_time: Option<DateTime<Utc>>,
    /// 用户注意力疲劳度估算（0.0-1.0）
    fatigue_level: f32,
}

impl EnhancedMicroBreakScheduler {
    /// 创建新的增强微休息调度器
    pub fn new(min_interval_minutes: u32, max_interval_minutes: u32, duration_seconds: u32) -> Self {
        Self {
            min_interval_minutes,
            max_interval_minutes,
            duration_seconds,
            next_break_at: None,
            break_count: 0,
            skip_count: 0,
            completed_count: 0,
            interval_adjustment_factor: 1.0,
            last_break_time: None,
            fatigue_level: 0.0,
        }
    }

    /// 开始智能调度（专注会话开始时调用）
    pub fn start_scheduling(&mut self) {
        self.break_count = 0;
        self.skip_count = 0;
        self.completed_count = 0;
        self.fatigue_level = 0.0;
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

    /// 微休息被触发时调用
    pub fn on_break_triggered(&mut self, current_elapsed: u64) {
        self.break_count += 1;
        self.last_break_time = Some(Utc::now());
        
        // 计算下次微休息时间
        let next_schedule_from = current_elapsed + self.duration_seconds as u64;
        self.schedule_next_break(next_schedule_from);
    }

    /// 微休息被跳过时调用
    pub fn on_break_skipped(&mut self, current_elapsed: u64) {
        self.skip_count += 1;
        
        // 跳过时缩短下次间隔（表示用户可能需要更频繁的提醒）
        self.adjust_interval_for_skip();
        
        let next_schedule_from = current_elapsed;
        self.schedule_next_break(next_schedule_from);
    }

    /// 微休息被完成时调用
    pub fn on_break_completed(&mut self, current_elapsed: u64) {
        self.completed_count += 1;
        
        // 完成时可以适当延长间隔（用户配合度高）
        self.adjust_interval_for_completion();
        
        let next_schedule_from = current_elapsed;
        self.schedule_next_break(next_schedule_from);
        
        // 重置疲劳度
        self.fatigue_level = (self.fatigue_level * 0.7).max(0.0);
    }

    /// 根据跳过行为调整间隔
    fn adjust_interval_for_skip(&mut self) {
        // 跳过时稍微缩短间隔，最多缩短到原来的70%
        self.interval_adjustment_factor = (self.interval_adjustment_factor * 0.9).max(0.7);
        
        // 增加疲劳度
        self.fatigue_level = (self.fatigue_level + 0.1).min(1.0);
    }

    /// 根据完成行为调整间隔
    fn adjust_interval_for_completion(&mut self) {
        // 完成时可以稍微延长间隔，最多延长到原来的130%
        self.interval_adjustment_factor = (self.interval_adjustment_factor * 1.05).min(1.3);
    }

    /// 智能计算下次微休息时间
    fn schedule_next_break(&mut self, from_seconds: u64) {
        let base_min = self.min_interval_minutes as f32 * 60.0;
        let base_max = self.max_interval_minutes as f32 * 60.0;
        
        // 应用调整因子
        let adjusted_min = (base_min * self.interval_adjustment_factor) as u64;
        let adjusted_max = (base_max * self.interval_adjustment_factor) as u64;
        
        // 根据疲劳度进一步调整（疲劳度高时缩短间隔）
        let fatigue_factor = 1.0 - (self.fatigue_level * 0.3);
        let final_min = (adjusted_min as f32 * fatigue_factor) as u64;
        let final_max = (adjusted_max as f32 * fatigue_factor) as u64;
        
        let mut rng = rand::thread_rng();
        let random_interval = rng.gen_range(final_min..=final_max);
        
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

    /// 获取跳过计数
    pub fn skip_count(&self) -> u32 {
        self.skip_count
    }

    /// 获取完成计数
    pub fn completed_count(&self) -> u32 {
        self.completed_count
    }

    /// 获取完成率
    pub fn completion_rate(&self) -> f32 {
        if self.break_count == 0 {
            return 1.0;
        }
        self.completed_count as f32 / self.break_count as f32
    }

    /// 获取当前疲劳度
    pub fn fatigue_level(&self) -> f32 {
        self.fatigue_level
    }

    /// 获取调整因子
    pub fn adjustment_factor(&self) -> f32 {
        self.interval_adjustment_factor
    }

    /// 获取统计摘要
    pub fn get_stats_summary(&self) -> MicroBreakStats {
        MicroBreakStats {
            total_triggered: self.break_count,
            completed: self.completed_count,
            skipped: self.skip_count,
            completion_rate: self.completion_rate(),
            fatigue_level: self.fatigue_level,
            adjustment_factor: self.interval_adjustment_factor,
            next_break_at: self.next_break_at,
        }
    }

    /// 重置调度器
    pub fn reset(&mut self) {
        self.next_break_at = None;
        self.break_count = 0;
        self.skip_count = 0;
        self.completed_count = 0;
        self.interval_adjustment_factor = 1.0;
        self.last_break_time = None;
        self.fatigue_level = 0.0;
    }
}

/// 微休息统计数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MicroBreakStats {
    /// 总触发次数
    pub total_triggered: u32,
    /// 完成次数
    pub completed: u32,
    /// 跳过次数
    pub skipped: u32,
    /// 完成率 (0.0-1.0)
    pub completion_rate: f32,
    /// 疲劳度 (0.0-1.0)
    pub fatigue_level: f32,
    /// 间隔调整因子
    pub adjustment_factor: f32,
    /// 下次微休息时间
    pub next_break_at: Option<u64>,
} 