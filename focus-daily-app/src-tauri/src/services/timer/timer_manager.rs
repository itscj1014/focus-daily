use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{interval, Duration};
use uuid::Uuid;
use tauri::{AppHandle, Emitter};
use rand::Rng;

use crate::models::{SessionType, UserSettings};
use super::timer_state::{TimerState, SessionPhase, TimerEvent};

/// 定时器管理器错误类型
#[derive(Debug, thiserror::Error)]
pub enum TimerError {
    #[error("定时器已在运行")]
    AlreadyRunning,
    #[error("定时器未运行")]
    NotRunning,
    #[error("数据库错误: {0}")]
    Database(String),
    #[error("设置错误: {0}")]
    Settings(String),
    #[error("状态错误: {0}")]
    InvalidState(String),
}

/// 会话循环状态
#[derive(Debug, Clone, PartialEq)]
pub enum CycleState {
    WaitingToStart,     // 等待开始
    InFocusSession,     // 专注会话中
    InLongBreak,        // 长休息中
    InMicroBreak,       // 微休息中
    Completed,          // 已完成
}

/// 定时器管理器
pub struct TimerManager {
    /// 当前定时器状态
    state: Arc<RwLock<TimerState>>,
    /// 用户设置
    settings: Arc<RwLock<UserSettings>>,
    /// Tauri应用句柄
    app_handle: AppHandle,
    /// 定时器任务句柄
    timer_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    /// 微休息调度器句柄
    micro_break_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    /// 当前会话循环状态
    cycle_state: Arc<RwLock<CycleState>>,
    /// 完成的专注会话数量
    completed_focus_sessions: Arc<Mutex<u32>>,
}

impl TimerManager {
    /// 创建新的定时器管理器
    pub fn new(app_handle: AppHandle, settings: UserSettings) -> Self {
        Self {
            state: Arc::new(RwLock::new(TimerState::default())),
            settings: Arc::new(RwLock::new(settings)),
            app_handle,
            timer_handle: Arc::new(Mutex::new(None)),
            micro_break_handle: Arc::new(Mutex::new(None)),
            cycle_state: Arc::new(RwLock::new(CycleState::WaitingToStart)),
            completed_focus_sessions: Arc::new(Mutex::new(0)),
        }
    }

    /// 获取当前状态
    pub async fn get_state(&self) -> TimerState {
        self.state.read().await.clone()
    }

    /// 获取循环状态
    pub async fn get_cycle_state(&self) -> CycleState {
        self.cycle_state.read().await.clone()
    }

    /// 更新用户设置
    pub async fn update_settings(&self, new_settings: UserSettings) {
        let mut settings = self.settings.write().await;
        *settings = new_settings;
    }

    /// 开始专注会话
    pub async fn start_focus_session(&self) -> Result<String, TimerError> {
        let mut state = self.state.write().await;
        let mut cycle_state = self.cycle_state.write().await;
        
        // 检查是否已有定时器在运行
        if state.is_running() {
            return Err(TimerError::AlreadyRunning);
        }

        // 检查循环状态
        if *cycle_state != CycleState::WaitingToStart && *cycle_state != CycleState::Completed {
            return Err(TimerError::InvalidState("只能在等待状态或完成状态启动专注会话".to_string()));
        }

        // 获取用户设置
        let settings = self.settings.read().await;
        let duration_minutes = settings.focus_duration_minutes as u32;
        
        // 创建新的专注会话状态
        let session_id = Uuid::new_v4().to_string();
        *state = TimerState::new_focus_session(duration_minutes);
        state.session_id = Some(session_id.clone());
        state.start();

        // 更新循环状态
        *cycle_state = CycleState::InFocusSession;

        // 计算下次微休息时间
        self.schedule_next_micro_break(&mut state, &settings).await;

        drop(state);
        drop(cycle_state);
        drop(settings);

        // 保存会话到数据库
        self.save_session_to_db(&session_id, SessionType::Focus, duration_minutes * 60).await?;

        // 启动定时器
        self.start_timer_task().await;

        // 启动微休息调度器
        self.start_micro_break_scheduler().await;

        // 发送开始事件
        self.emit_event(TimerEvent::Started {
            phase: SessionPhase::Focus,
            duration: (duration_minutes * 60) as u64,
            session_id: session_id.clone(),
        }).await;

        Ok(session_id)
    }

    /// 开始长休息会话
    pub async fn start_long_break_session(&self) -> Result<String, TimerError> {
        let mut state = self.state.write().await;
        let mut cycle_state = self.cycle_state.write().await;
        
        // 检查是否已有定时器在运行
        if state.is_running() {
            return Err(TimerError::AlreadyRunning);
        }

        let settings = self.settings.read().await;
        let duration_minutes = settings.long_break_duration_minutes as u32;
        
        // 创建新的长休息状态
        let session_id = Uuid::new_v4().to_string();
        *state = TimerState::new_long_break_session(duration_minutes);
        state.session_id = Some(session_id.clone());
        state.start();

        // 更新循环状态
        *cycle_state = CycleState::InLongBreak;

        drop(state);
        drop(cycle_state);
        drop(settings);

        // 保存会话到数据库
        self.save_session_to_db(&session_id, SessionType::LongBreak, duration_minutes * 60).await?;
        
        // 启动定时器（长休息不需要微休息调度器）
        self.start_timer_task().await;

        // 发送开始事件
        self.emit_event(TimerEvent::Started {
            phase: SessionPhase::LongBreak,
            duration: (duration_minutes * 60) as u64,
            session_id: session_id.clone(),
        }).await;

        Ok(session_id)
    }

    /// 开始微休息会话
    pub async fn start_micro_break_session(&self) -> Result<String, TimerError> {
        let mut state = self.state.write().await;
        let mut cycle_state = self.cycle_state.write().await;
        
        let settings = self.settings.read().await;
        let duration_seconds = settings.micro_break_duration_seconds as u32;
        
        // 创建新的微休息状态
        let session_id = Uuid::new_v4().to_string();
        *state = TimerState::new_micro_break_session(duration_seconds);
        state.session_id = Some(session_id.clone());
        state.start();

        // 更新循环状态
        *cycle_state = CycleState::InMicroBreak;

        // 增加微休息计数
        state.micro_break_count += 1;

        drop(state);
        drop(cycle_state);
        drop(settings);

        // 启动微休息定时器
        self.start_timer_task().await;

        // 发送微休息开始事件
        self.emit_event(TimerEvent::Started {
            phase: SessionPhase::MicroBreak,
            duration: duration_seconds as u64,
            session_id: session_id.clone(),
        }).await;

        Ok(session_id)
    }

    /// 跳过微休息
    pub async fn skip_micro_break(&self) -> Result<(), TimerError> {
        let mut state = self.state.write().await;
        let mut cycle_state = self.cycle_state.write().await;
        
        // 检查当前是否在微休息状态
        if *cycle_state != CycleState::InMicroBreak {
            return Err(TimerError::InvalidState("当前不在微休息状态".to_string()));
        }

        // 停止定时器
        self.stop_timer_task().await;

        // 完成微休息
        state.complete();
        
        // 返回专注会话
        *cycle_state = CycleState::InFocusSession;
        
        // 重新创建专注会话状态
        let settings = self.settings.read().await;
        let focus_duration = settings.focus_duration_minutes as u32;
        let remaining_focus_time = (focus_duration * 60) as u64 - state.elapsed_duration;
        
        *state = TimerState::new_focus_session(focus_duration);
        state.remaining_duration = remaining_focus_time;
        state.elapsed_duration = (focus_duration * 60) as u64 - remaining_focus_time;
        state.start();

        drop(state);
        drop(cycle_state);
        drop(settings);

        // 重新启动专注定时器和微休息调度器
        self.start_timer_task().await;
        self.start_micro_break_scheduler().await;

        // 发送跳过事件
        self.emit_event(TimerEvent::PhaseChanged {
            from: SessionPhase::MicroBreak,
            to: SessionPhase::Focus,
        }).await;

        Ok(())
    }

    /// 暂停定时器
    pub async fn pause_timer(&self) -> Result<(), TimerError> {
        let mut state = self.state.write().await;
        
        if !state.is_running() {
            return Err(TimerError::NotRunning);
        }

        state.pause();
        let phase = state.phase.clone();
        let remaining = state.remaining_duration;

        drop(state);

        // 停止定时器任务
        self.stop_timer_task().await;

        self.emit_event(TimerEvent::Paused {
            phase,
            remaining,
        }).await;

        Ok(())
    }

    /// 恢复定时器
    pub async fn resume_timer(&self) -> Result<(), TimerError> {
        let mut state = self.state.write().await;
        
        if !state.is_paused() {
            return Err(TimerError::NotRunning);
        }

        state.resume();
        let phase = state.phase.clone();
        let remaining = state.remaining_duration;

        drop(state);

        // 重新启动定时器任务
        self.start_timer_task().await;

        // 如果是专注阶段，重新启动微休息调度器
        if phase == SessionPhase::Focus {
            self.start_micro_break_scheduler().await;
        }

        self.emit_event(TimerEvent::Resumed {
            phase,
            remaining,
        }).await;

        Ok(())
    }

    /// 重置定时器
    pub async fn reset_timer(&self) -> Result<(), TimerError> {
        let mut state = self.state.write().await;
        let phase = state.phase.clone();
        
        state.reset();
        drop(state);

        // 停止所有定时器任务
        self.stop_timer_task().await;
        self.stop_micro_break_scheduler().await;

        self.emit_event(TimerEvent::Reset { phase }).await;

        Ok(())
    }

    /// 启动定时器任务
    async fn start_timer_task(&self) {
        // 停止现有任务
        self.stop_timer_task().await;

        let state = Arc::clone(&self.state);
        let cycle_state = Arc::clone(&self.cycle_state);
        let settings = Arc::clone(&self.settings);
        let app_handle = self.app_handle.clone();
        let completed_sessions = Arc::clone(&self.completed_focus_sessions);

        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            
            loop {
                interval.tick().await;
                
                let mut state_guard = state.write().await;
                
                if !state_guard.is_running() {
                    break;
                }

                if state_guard.remaining_duration > 0 {
                    state_guard.remaining_duration -= 1;
                    state_guard.elapsed_duration += 1;
                    
                    let phase = state_guard.phase.clone();
                    let remaining = state_guard.remaining_duration;
                    let elapsed = state_guard.elapsed_duration;
                    let progress = state_guard.progress();
                    
                    drop(state_guard);

                    // 发送tick事件
                    let _ = app_handle.emit("timer-tick", TimerEvent::Tick {
                        phase,
                        remaining,
                        elapsed,
                        progress,
                    });
                } else {
                    // 定时器完成 - 处理自动状态转换
                    let phase = state_guard.phase.clone();
                    let session_id = state_guard.session_id.clone().unwrap_or_default();
                    state_guard.complete();
                    
                    let mut cycle_guard = cycle_state.write().await;
                    
                    drop(state_guard);
                    
                    // 发送事件到前端更新数据库中的会话完成状态
                    let update_data = serde_json::json!({
                        "session_id": session_id.clone(),
                        "completed": true,
                        "end_time": chrono::Utc::now().to_rfc3339(),
                        "updated_at": chrono::Utc::now().to_rfc3339()
                    });
                    let _ = app_handle.emit("update-session-completion", update_data);
                    
                    // 根据当前阶段处理状态转换
                    match phase {
                        SessionPhase::Focus => {
                            // 专注会话完成，增加计数并切换到长休息
                            let mut sessions = completed_sessions.lock().await;
                            *sessions += 1;
                            drop(sessions);
                            
                            *cycle_guard = CycleState::WaitingToStart; // 等待用户决定是否开始长休息
                            
                            // 发送完成事件和阶段变更事件
                            let _ = app_handle.emit("timer-completed", TimerEvent::Completed {
                                phase: SessionPhase::Focus,
                                session_id: session_id.clone(),
                            });
                            
                            let _ = app_handle.emit("focus-session-completed", serde_json::json!({
                                "session_id": session_id,
                                "completed_sessions": *completed_sessions.lock().await,
                                "next_phase": "long_break"
                            }));
                        },
                        SessionPhase::LongBreak => {
                            // 长休息完成，回到等待状态
                            *cycle_guard = CycleState::WaitingToStart;
                            
                            let _ = app_handle.emit("timer-completed", TimerEvent::Completed {
                                phase: SessionPhase::LongBreak,
                                session_id: session_id.clone(),
                            });
                            
                            let _ = app_handle.emit("long-break-completed", serde_json::json!({
                                "session_id": session_id,
                                "next_phase": "focus"
                            }));
                        },
                        SessionPhase::MicroBreak => {
                            // 微休息完成，自动返回专注会话
                            *cycle_guard = CycleState::InFocusSession;
                            
                            let _ = app_handle.emit("timer-completed", TimerEvent::Completed {
                                phase: SessionPhase::MicroBreak,
                                session_id: session_id.clone(),
                            });
                            
                            // 自动恢复专注会话的逻辑需要在这里实现
                            let _ = app_handle.emit("micro-break-completed", serde_json::json!({
                                "session_id": session_id,
                                "returning_to_focus": true
                            }));
                        }
                    }
                    
                    drop(cycle_guard);
                    break;
                }
            }
        });

        let mut timer_handle = self.timer_handle.lock().await;
        *timer_handle = Some(handle);
    }

    /// 停止定时器任务
    async fn stop_timer_task(&self) {
        let mut timer_handle = self.timer_handle.lock().await;
        if let Some(handle) = timer_handle.take() {
            handle.abort();
        }
    }

    /// 启动微休息调度器
    async fn start_micro_break_scheduler(&self) {
        // 停止现有调度器
        self.stop_micro_break_scheduler().await;

        let state = Arc::clone(&self.state);
        let cycle_state = Arc::clone(&self.cycle_state);
        let settings = Arc::clone(&self.settings);
        let app_handle = self.app_handle.clone();

        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            
            loop {
                interval.tick().await;
                
                let state_guard = state.read().await;
                let cycle_guard = cycle_state.read().await;
                
                // 只在专注阶段运行
                if *cycle_guard != CycleState::InFocusSession || !state_guard.is_running() {
                    drop(state_guard);
                    drop(cycle_guard);
                    break;
                }

                // 检查是否到了微休息时间
                if let Some(next_break_at) = state_guard.next_micro_break_at {
                    if state_guard.elapsed_duration >= next_break_at {
                        let elapsed = state_guard.elapsed_duration;
                        let micro_break_count = state_guard.micro_break_count;
                        
                        drop(state_guard);
                        drop(cycle_guard);
                        
                        // 获取微休息设置
                        let settings_guard = settings.read().await;
                        let duration = settings_guard.micro_break_duration_seconds as u64;
                        drop(settings_guard);

                        // 触发微休息事件，前端可以选择是否开始微休息
                        let _ = app_handle.emit("micro-break-triggered", TimerEvent::MicroBreakTriggered {
                            count: micro_break_count + 1,
                            duration,
                        });
                        
                        // 发送微休息准备事件，包含更多上下文信息
                        let _ = app_handle.emit("micro-break-ready", serde_json::json!({
                            "count": micro_break_count + 1,
                            "duration": duration,
                            "focus_elapsed": elapsed,
                            "auto_start": true // 可以配置是否自动开始
                        }));
                        
                        break;
                    }
                }
                
                drop(state_guard);
                drop(cycle_guard);
            }
        });

        let mut micro_break_handle = self.micro_break_handle.lock().await;
        *micro_break_handle = Some(handle);
    }

    /// 停止微休息调度器
    async fn stop_micro_break_scheduler(&self) {
        let mut micro_break_handle = self.micro_break_handle.lock().await;
        if let Some(handle) = micro_break_handle.take() {
            handle.abort();
        }
    }

    /// 计算下次微休息时间
    async fn schedule_next_micro_break(&self, state: &mut TimerState, settings: &UserSettings) {
        let min_interval = settings.micro_break_min_interval_minutes as u64 * 60;
        let max_interval = settings.micro_break_max_interval_minutes as u64 * 60;
        
        let mut rng = rand::thread_rng();
        let random_interval = rng.gen_range(min_interval..=max_interval);
        
        state.next_micro_break_at = Some(state.elapsed_duration + random_interval);
    }

    /// 保存会话到数据库
    async fn save_session_to_db(&self, session_id: &str, session_type: SessionType, duration_seconds: u32) -> Result<(), TimerError> {
        // 在Tauri 2.0中，数据库操作应该通过前端JavaScript进行
        // 这里我们发送事件到前端，让前端处理数据库保存
        let session_data = serde_json::json!({
            "id": session_id,
            "session_type": match session_type {
                SessionType::Focus => "Focus",
                SessionType::LongBreak => "LongBreak", 
                SessionType::MicroBreak => "MicroBreak",
            },
            "duration_seconds": duration_seconds,
            "start_time": chrono::Utc::now().to_rfc3339(),
            "created_at": chrono::Utc::now().to_rfc3339(),
            "updated_at": chrono::Utc::now().to_rfc3339()
        });

        let _ = self.app_handle.emit("save-session-to-db", session_data);
        Ok(())
    }

    /// 更新会话完成状态
    async fn update_session_completion(&self, session_id: &str, completed: bool) -> Result<(), TimerError> {
        // 发送事件到前端处理数据库更新
        let update_data = serde_json::json!({
            "session_id": session_id,
            "completed": completed,
            "end_time": chrono::Utc::now().to_rfc3339(),
            "updated_at": chrono::Utc::now().to_rfc3339()
        });

        let _ = self.app_handle.emit("update-session-completion", update_data);
        Ok(())
    }

    /// 获取今日会话统计
    pub async fn get_today_stats(&self) -> Result<serde_json::Value, TimerError> {
        // 发送事件到前端请求统计数据
        let _ = self.app_handle.emit("request-today-stats", serde_json::json!({}));
        
        // 这里应该通过某种机制等待前端返回结果
        // 为了简化，先返回空的统计数据
        Ok(serde_json::json!({
            "date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
            "sessions": []
        }))
    }

    /// 发送事件到前端
    async fn emit_event(&self, event: TimerEvent) {
        let _ = self.app_handle.emit("timer-event", &event);
    }
} 