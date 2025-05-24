use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock, mpsc};
use tauri::{AppHandle, Emitter};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::services::timer::timer_state::TimerEvent;

/// 事件优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// 增强的事件包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedEvent {
    /// 事件ID
    pub id: String,
    /// 事件内容
    pub event: TimerEvent,
    /// 优先级
    pub priority: EventPriority,
    /// 创建时间
    pub timestamp: DateTime<Utc>,
    /// 事件类别
    pub category: EventCategory,
    /// 重试次数
    pub retry_count: u32,
    /// 是否需要持久化
    pub persist: bool,
}

/// 事件类别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventCategory {
    Timer,         // 定时器事件
    MicroBreak,    // 微休息事件
    Audio,         // 音频事件
    System,        // 系统事件
    Notification,  // 通知事件
    Analytics,     // 分析事件
}

/// 事件处理结果
#[derive(Debug, Clone)]
pub enum EventProcessResult {
    Success,
    Retry(Duration),  // 需要重试，指定延迟时间
    Failed(String),   // 处理失败，错误信息
    Discard,         // 丢弃事件
}

/// 事件统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStats {
    pub total_events: u64,
    pub successful_events: u64,
    pub failed_events: u64,
    pub discarded_events: u64,
    pub average_processing_time: f64,
    pub events_per_category: HashMap<String, u64>,
    pub events_per_priority: HashMap<String, u64>,
    pub last_updated: DateTime<Utc>,
}

/// 增强的事件管理器
pub struct EnhancedEventManager {
    /// Tauri应用句柄
    app_handle: AppHandle,
    /// 事件队列 (按优先级排序)
    event_queue: Arc<Mutex<VecDeque<EnhancedEvent>>>,
    /// 事件历史记录
    event_history: Arc<RwLock<VecDeque<EnhancedEvent>>>,
    /// 事件统计
    stats: Arc<RwLock<EventStats>>,
    /// 事件发送通道
    event_sender: mpsc::UnboundedSender<EnhancedEvent>,
    /// 配置参数
    config: EventManagerConfig,
}

/// 事件管理器配置
#[derive(Debug, Clone)]
pub struct EventManagerConfig {
    /// 最大队列大小
    pub max_queue_size: usize,
    /// 最大历史记录数量
    pub max_history_size: usize,
    /// 批处理大小
    pub batch_size: usize,
    /// 批处理间隔
    pub batch_interval: Duration,
    /// 最大重试次数
    pub max_retries: u32,
    /// 事件过期时间
    pub event_ttl: Duration,
    /// 是否启用事件去重
    pub enable_deduplication: bool,
}

impl Default for EventManagerConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 1000,
            max_history_size: 500,
            batch_size: 10,
            batch_interval: Duration::from_millis(100),
            max_retries: 3,
            event_ttl: Duration::from_secs(60),
            enable_deduplication: true,
        }
    }
}

/// 音频事件处理器特征 (修复为非async版本)
pub trait AudioEventHandler {
    fn handle_audio_event(&self, event: &AudioEvent) -> Result<(), String>;
}

/// 音频事件类型 (为Week 4准备)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioEvent {
    PlayFocusStart,
    PlayFocusEnd,
    PlayLongBreakStart,
    PlayLongBreakEnd,
    PlayMicroBreakStart,
    PlayMicroBreakEnd,
    PlayNotification { sound_type: String },
    SetVolume { level: f32 },
    Mute,
    Unmute,
    ConfigChanged { config: AudioConfig },
}

/// 音频配置 (为Week 4准备)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub enabled: bool,
    pub volume: f32,
    pub focus_start_sound: Option<String>,
    pub focus_end_sound: Option<String>,
    pub break_start_sound: Option<String>,
    pub break_end_sound: Option<String>,
    pub notification_sound: Option<String>,
}

impl EnhancedEventManager {
    /// 创建新的增强事件管理器
    pub fn new(app_handle: AppHandle, config: Option<EventManagerConfig>) -> Self {
        let config = config.unwrap_or_default();
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let manager = Self {
            app_handle: app_handle.clone(),
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            event_history: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(EventStats::default())),
            event_sender: sender,
            config,
        };
        
        // 启动事件处理任务
        manager.start_event_processor(receiver);
        
        manager
    }
    
    /// 发送事件
    pub async fn emit_event(&self, event: TimerEvent, priority: Option<EventPriority>) -> Result<(), String> {
        let enhanced_event = EnhancedEvent {
            id: uuid::Uuid::new_v4().to_string(),
            priority: priority.unwrap_or(EventPriority::Normal),
            timestamp: Utc::now(),
            category: self.categorize_event(&event),
            retry_count: 0,
            persist: self.should_persist_event(&event),
            event,
        };
        
        self.event_sender.send(enhanced_event)
            .map_err(|e| format!("Failed to send event: {}", e))?;
        
        Ok(())
    }
    
    /// 发送音频事件 (简化实现，移除async handler调用)
    pub async fn emit_audio_event(&self, audio_event: AudioEvent, priority: Option<EventPriority>) -> Result<(), String> {
        // 直接转换为TimerEvent并发送到前端
        let timer_event = self.audio_event_to_timer_event(audio_event);
        self.emit_event(timer_event, priority).await?;
        
        Ok(())
    }
    
    /// 发送批量事件
    pub async fn emit_batch_events(&self, events: Vec<(TimerEvent, Option<EventPriority>)>) -> Result<(), String> {
        for (event, priority) in events {
            self.emit_event(event, priority).await?;
        }
        Ok(())
    }
    
    /// 获取事件统计
    pub async fn get_stats(&self) -> EventStats {
        self.stats.read().await.clone()
    }
    
    /// 获取事件历史
    pub async fn get_event_history(&self, limit: Option<usize>) -> Vec<EnhancedEvent> {
        let history = self.event_history.read().await;
        let limit = limit.unwrap_or(100).min(history.len());
        history.iter().rev().take(limit).cloned().collect()
    }
    
    /// 清理过期事件
    pub async fn cleanup_expired_events(&self) {
        let now = Utc::now();
        let ttl = self.config.event_ttl;
        
        // 清理队列中的过期事件
        let mut queue = self.event_queue.lock().await;
        queue.retain(|event| {
            now.signed_duration_since(event.timestamp).to_std().unwrap_or(Duration::ZERO) < ttl
        });
        
        // 清理历史记录
        let mut history = self.event_history.write().await;
        while history.len() > self.config.max_history_size {
            history.pop_front();
        }
    }
    
    /// 获取队列状态
    pub async fn get_queue_status(&self) -> (usize, usize) {
        let queue = self.event_queue.lock().await;
        (queue.len(), self.config.max_queue_size)
    }
    
    /// 启动事件处理器
    fn start_event_processor(&self, mut receiver: mpsc::UnboundedReceiver<EnhancedEvent>) {
        let app_handle = self.app_handle.clone();
        let event_queue = Arc::clone(&self.event_queue);
        let event_history = Arc::clone(&self.event_history);
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut batch = Vec::new();
            let mut batch_timer = tokio::time::interval(config.batch_interval);
            
            loop {
                tokio::select! {
                    // 接收新事件
                    Some(event) = receiver.recv() => {
                        batch.push(event);
                        
                        // 如果批次达到大小限制，立即处理
                        if batch.len() >= config.batch_size {
                            Self::process_event_batch(
                                &app_handle,
                                &mut batch,
                                &event_queue,
                                &event_history,
                                &stats,
                                &config
                            ).await;
                        }
                    }
                    
                    // 批处理定时器
                    _ = batch_timer.tick() => {
                        if !batch.is_empty() {
                            Self::process_event_batch(
                                &app_handle,
                                &mut batch,
                                &event_queue,
                                &event_history,
                                &stats,
                                &config
                            ).await;
                        }
                    }
                }
            }
        });
    }
    
    /// 处理事件批次
    async fn process_event_batch(
        app_handle: &AppHandle,
        batch: &mut Vec<EnhancedEvent>,
        event_queue: &Arc<Mutex<VecDeque<EnhancedEvent>>>,
        event_history: &Arc<RwLock<VecDeque<EnhancedEvent>>>,
        stats: &Arc<RwLock<EventStats>>,
        config: &EventManagerConfig,
    ) {
        if batch.is_empty() {
            return;
        }
        
        // 按优先级排序
        batch.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        // 处理每个事件
        for event in batch.drain(..) {
            let start_time = Instant::now();
            let result = Self::process_single_event(app_handle, &event).await;
            let processing_time = start_time.elapsed();
            
            // 更新统计信息
            Self::update_stats(stats, &event, &result, processing_time).await;
            
            // 根据处理结果决定下一步
            match result {
                EventProcessResult::Success => {
                    // 成功处理，添加到历史记录
                    let mut history = event_history.write().await;
                    history.push_back(event);
                    if history.len() > config.max_history_size {
                        history.pop_front();
                    }
                }
                EventProcessResult::Retry(delay) => {
                    if event.retry_count < config.max_retries {
                        // 重新排队，增加重试计数
                        let mut retry_event = event;
                        retry_event.retry_count += 1;
                        
                        // 延迟后重新加入队列
                        let queue = Arc::clone(event_queue);
                        tokio::spawn(async move {
                            tokio::time::sleep(delay).await;
                            let mut q = queue.lock().await;
                            q.push_back(retry_event);
                        });
                    } else {
                        // 超过最大重试次数，记录失败
                        eprintln!("Event {} failed after {} retries", event.id, event.retry_count);
                    }
                }
                EventProcessResult::Failed(error) => {
                    eprintln!("Event {} processing failed: {}", event.id, error);
                }
                EventProcessResult::Discard => {
                    // 事件被丢弃，无需处理
                }
            }
        }
    }
    
    /// 处理单个事件
    async fn process_single_event(app_handle: &AppHandle, event: &EnhancedEvent) -> EventProcessResult {
        // 事件去重检查
        if Self::is_duplicate_event(event).await {
            return EventProcessResult::Discard;
        }
        
        // 发送到前端
        match Self::emit_to_frontend(app_handle, event).await {
            Ok(_) => EventProcessResult::Success,
            Err(e) => {
                // 根据错误类型决定是否重试
                if e.contains("connection") || e.contains("timeout") {
                    EventProcessResult::Retry(Duration::from_millis(500))
                } else {
                    EventProcessResult::Failed(e)
                }
            }
        }
    }
    
    /// 发送事件到前端
    async fn emit_to_frontend(app_handle: &AppHandle, event: &EnhancedEvent) -> Result<(), String> {
        // 发送具体事件类型
        let event_name = Self::get_event_name(&event.event);
        app_handle.emit(&event_name, &event.event)
            .map_err(|e| format!("Failed to emit {}: {}", event_name, e))?;
        
        // 发送通用事件
        app_handle.emit("timer-event", &event.event)
            .map_err(|e| format!("Failed to emit timer-event: {}", e))?;
        
        // 发送增强事件信息
        app_handle.emit("enhanced-event", event)
            .map_err(|e| format!("Failed to emit enhanced-event: {}", e))?;
        
        Ok(())
    }
    
    /// 更新统计信息
    async fn update_stats(
        stats: &Arc<RwLock<EventStats>>,
        event: &EnhancedEvent,
        result: &EventProcessResult,
        processing_time: Duration,
    ) {
        let mut stats_guard = stats.write().await;
        
        stats_guard.total_events += 1;
        
        match result {
            EventProcessResult::Success => stats_guard.successful_events += 1,
            EventProcessResult::Failed(_) => stats_guard.failed_events += 1,
            EventProcessResult::Discard => stats_guard.discarded_events += 1,
            EventProcessResult::Retry(_) => {} // 重试不计入统计
        }
        
        // 更新分类统计
        let category_name = format!("{:?}", event.category);
        *stats_guard.events_per_category.entry(category_name).or_insert(0) += 1;
        
        let priority_name = format!("{:?}", event.priority);
        *stats_guard.events_per_priority.entry(priority_name).or_insert(0) += 1;
        
        // 更新平均处理时间
        let total_time = stats_guard.average_processing_time * (stats_guard.total_events - 1) as f64;
        stats_guard.average_processing_time = (total_time + processing_time.as_secs_f64()) / stats_guard.total_events as f64;
        
        stats_guard.last_updated = Utc::now();
    }
    
    /// 检查是否为重复事件
    async fn is_duplicate_event(_event: &EnhancedEvent) -> bool {
        // TODO: 实现事件去重逻辑
        false
    }
    
    /// 获取事件名称
    fn get_event_name(event: &TimerEvent) -> String {
        match event {
            TimerEvent::Started { .. } => "timer-started".to_string(),
            TimerEvent::Paused { .. } => "timer-paused".to_string(),
            TimerEvent::Resumed { .. } => "timer-resumed".to_string(),
            TimerEvent::Tick { .. } => "timer-tick".to_string(),
            TimerEvent::Completed { .. } => "timer-completed".to_string(),
            TimerEvent::Reset { .. } => "timer-reset".to_string(),
            TimerEvent::MicroBreakTriggered { .. } => "micro-break-triggered".to_string(),
            TimerEvent::MicroBreakSkipped { .. } => "micro-break-skipped".to_string(),
            TimerEvent::MicroBreakCompleted { .. } => "micro-break-completed".to_string(),
            TimerEvent::MicroBreakSkipLimitReached { .. } => "micro-break-skip-limit-reached".to_string(),
            TimerEvent::MicroBreakScheduleUpdated { .. } => "micro-break-schedule-updated".to_string(),
            TimerEvent::MicroBreakStatsUpdated { .. } => "micro-break-stats-updated".to_string(),
            TimerEvent::PhaseChanged { .. } => "phase-changed".to_string(),
            TimerEvent::FatigueWarning { .. } => "fatigue-warning".to_string(),
            TimerEvent::EfficiencyFeedback { .. } => "efficiency-feedback".to_string(),
        }
    }
    
    /// 事件分类
    fn categorize_event(&self, event: &TimerEvent) -> EventCategory {
        match event {
            TimerEvent::Started { .. } |
            TimerEvent::Paused { .. } |
            TimerEvent::Resumed { .. } |
            TimerEvent::Tick { .. } |
            TimerEvent::Completed { .. } |
            TimerEvent::Reset { .. } => EventCategory::Timer,
            
            TimerEvent::MicroBreakTriggered { .. } |
            TimerEvent::MicroBreakSkipped { .. } |
            TimerEvent::MicroBreakCompleted { .. } |
            TimerEvent::MicroBreakSkipLimitReached { .. } |
            TimerEvent::MicroBreakScheduleUpdated { .. } |
            TimerEvent::MicroBreakStatsUpdated { .. } => EventCategory::MicroBreak,
            
            TimerEvent::PhaseChanged { .. } => EventCategory::System,
            TimerEvent::FatigueWarning { .. } => EventCategory::Notification,
            TimerEvent::EfficiencyFeedback { .. } => EventCategory::Analytics,
        }
    }
    
    /// 判断是否需要持久化事件
    fn should_persist_event(&self, event: &TimerEvent) -> bool {
        match event {
            TimerEvent::Started { .. } |
            TimerEvent::Completed { .. } |
            TimerEvent::MicroBreakCompleted { .. } |
            TimerEvent::EfficiencyFeedback { .. } => true,
            _ => false,
        }
    }
    
    /// 转换音频事件为定时器事件
    fn audio_event_to_timer_event(&self, audio_event: AudioEvent) -> TimerEvent {
        // 创建一个表示音频事件的TimerEvent
        // 这里使用PhaseChanged作为载体，实际应用中可能需要扩展TimerEvent
        match audio_event {
            AudioEvent::PlayFocusStart => TimerEvent::PhaseChanged {
                from: crate::services::timer::timer_state::SessionPhase::Focus,
                to: crate::services::timer::timer_state::SessionPhase::Focus,
            },
            // 其他音频事件的转换...
            _ => TimerEvent::PhaseChanged {
                from: crate::services::timer::timer_state::SessionPhase::Focus,
                to: crate::services::timer::timer_state::SessionPhase::Focus,
            },
        }
    }
}

impl Default for EventStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            successful_events: 0,
            failed_events: 0,
            discarded_events: 0,
            average_processing_time: 0.0,
            events_per_category: HashMap::new(),
            events_per_priority: HashMap::new(),
            last_updated: Utc::now(),
        }
    }
} 