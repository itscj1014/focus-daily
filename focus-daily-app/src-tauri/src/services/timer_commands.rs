use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::RwLock;

use crate::models::UserSettings;
use crate::services::timer::{TimerManager, TimerState};
use crate::services::timer::timer_manager::CycleState;
use crate::services::events::{
    EnhancedEventManager, EventManagerConfig, EventStats, EventPriority,
    PerformanceMonitor, PerformanceReport, SystemHealth, PerformanceAlert,
    AudioEvent, AudioConfig,
};

/// 全局定时器管理器状态
pub type TimerManagerState = Arc<RwLock<Option<TimerManager>>>;

/// 全局增强事件管理器状态
pub type EventManagerState = Arc<RwLock<Option<EnhancedEventManager>>>;

/// 全局性能监控器状态
pub type PerformanceMonitorState = Arc<RwLock<Option<PerformanceMonitor>>>;

/// 初始化定时器管理器
#[tauri::command]
pub async fn init_timer_manager(
    app_handle: AppHandle,
    timer_manager: State<'_, TimerManagerState>,
    event_manager: State<'_, EventManagerState>,
    performance_monitor: State<'_, PerformanceMonitorState>,
) -> Result<(), String> {
    // 获取默认用户设置
    let default_settings = UserSettings::default();
    
    // 创建定时器管理器
    let manager = TimerManager::new(app_handle.clone(), default_settings);
    
    // 创建增强事件管理器
    let enhanced_event_manager = EnhancedEventManager::new(app_handle.clone(), None);
    
    // 创建性能监控器
    let perf_monitor = PerformanceMonitor::new(None);
    
    // 存储到全局状态
    let mut manager_guard = timer_manager.write().await;
    *manager_guard = Some(manager);
    
    let mut event_guard = event_manager.write().await;
    *event_guard = Some(enhanced_event_manager);
    
    let mut perf_guard = performance_monitor.write().await;
    *perf_guard = Some(perf_monitor);
    
    Ok(())
}

/// 获取定时器当前状态
#[tauri::command]
pub async fn get_timer_state(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<TimerState, String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        Ok(manager.get_state().await)
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 获取循环状态
#[tauri::command]
pub async fn get_cycle_state(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<String, String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        let cycle_state = manager.get_cycle_state().await;
        Ok(format!("{:?}", cycle_state))
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 开始专注会话
#[tauri::command]
pub async fn start_focus_session(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<String, String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.start_focus_session().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 开始长休息会话
#[tauri::command]
pub async fn start_long_break_session(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<String, String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.start_long_break_session().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 开始微休息会话
#[tauri::command]
pub async fn start_micro_break_session(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<String, String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.start_micro_break_session().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 暂停定时器
#[tauri::command]
pub async fn pause_timer(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<(), String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.pause_timer().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 恢复定时器
#[tauri::command]
pub async fn resume_timer(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<(), String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.resume_timer().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 重置定时器
#[tauri::command]
pub async fn reset_timer(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<(), String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.reset_timer().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 跳过微休息
#[tauri::command]
pub async fn skip_micro_break(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<(), String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.skip_micro_break().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 获取今日统计
#[tauri::command]
pub async fn get_today_stats(
    timer_manager: State<'_, TimerManagerState>,
) -> Result<serde_json::Value, String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.get_today_stats().await
            .map_err(|e| e.to_string())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

/// 更新用户设置
#[tauri::command]
pub async fn update_timer_settings(
    timer_manager: State<'_, TimerManagerState>,
    settings: UserSettings,
) -> Result<(), String> {
    let manager_guard = timer_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.update_settings(settings).await;
        Ok(())
    } else {
        Err("定时器管理器未初始化".to_string())
    }
}

// ============ Day 4 增强功能命令 ============

/// 获取事件统计信息
#[tauri::command]
pub async fn get_event_stats(
    event_manager: State<'_, EventManagerState>,
) -> Result<EventStats, String> {
    let manager_guard = event_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        Ok(manager.get_stats().await)
    } else {
        Err("事件管理器未初始化".to_string())
    }
}

/// 获取事件历史记录
#[tauri::command]
pub async fn get_event_history(
    event_manager: State<'_, EventManagerState>,
    limit: Option<usize>,
) -> Result<serde_json::Value, String> {
    let manager_guard = event_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        let history = manager.get_event_history(limit).await;
        Ok(serde_json::to_value(history).unwrap())
    } else {
        Err("事件管理器未初始化".to_string())
    }
}

/// 获取事件队列状态
#[tauri::command]
pub async fn get_event_queue_status(
    event_manager: State<'_, EventManagerState>,
) -> Result<(usize, usize), String> {
    let manager_guard = event_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        Ok(manager.get_queue_status().await)
    } else {
        Err("事件管理器未初始化".to_string())
    }
}

/// 清理过期事件
#[tauri::command]
pub async fn cleanup_expired_events(
    event_manager: State<'_, EventManagerState>,
) -> Result<(), String> {
    let manager_guard = event_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        manager.cleanup_expired_events().await;
        Ok(())
    } else {
        Err("事件管理器未初始化".to_string())
    }
}

/// 获取性能报告
#[tauri::command]
pub async fn get_performance_report(
    performance_monitor: State<'_, PerformanceMonitorState>,
) -> Result<PerformanceReport, String> {
    let monitor_guard = performance_monitor.read().await;
    
    if let Some(monitor) = monitor_guard.as_ref() {
        Ok(monitor.get_performance_report().await)
    } else {
        Err("性能监控器未初始化".to_string())
    }
}

/// 获取系统健康状态
#[tauri::command]
pub async fn get_system_health(
    performance_monitor: State<'_, PerformanceMonitorState>,
) -> Result<SystemHealth, String> {
    let monitor_guard = performance_monitor.read().await;
    
    if let Some(monitor) = monitor_guard.as_ref() {
        Ok(monitor.get_system_health().await)
    } else {
        Err("性能监控器未初始化".to_string())
    }
}

/// 获取活跃警报
#[tauri::command]
pub async fn get_active_alerts(
    performance_monitor: State<'_, PerformanceMonitorState>,
) -> Result<Vec<PerformanceAlert>, String> {
    let monitor_guard = performance_monitor.read().await;
    
    if let Some(monitor) = monitor_guard.as_ref() {
        Ok(monitor.get_active_alerts().await)
    } else {
        Err("性能监控器未初始化".to_string())
    }
}

/// 解决性能警报
#[tauri::command]
pub async fn resolve_performance_alert(
    performance_monitor: State<'_, PerformanceMonitorState>,
    alert_id: String,
) -> Result<(), String> {
    let monitor_guard = performance_monitor.read().await;
    
    if let Some(monitor) = monitor_guard.as_ref() {
        monitor.resolve_alert(&alert_id).await;
        Ok(())
    } else {
        Err("性能监控器未初始化".to_string())
    }
}

/// 播放音频事件 (为Week 4准备)
#[tauri::command]
pub async fn play_audio_event(
    event_manager: State<'_, EventManagerState>,
    audio_event: AudioEvent,
    priority: Option<String>,
) -> Result<(), String> {
    let manager_guard = event_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        let event_priority = match priority.as_deref() {
            Some("low") => Some(EventPriority::Low),
            Some("high") => Some(EventPriority::High),
            Some("critical") => Some(EventPriority::Critical),
            _ => Some(EventPriority::Normal),
        };
        
        manager.emit_audio_event(audio_event, event_priority).await
    } else {
        Err("事件管理器未初始化".to_string())
    }
}

/// 更新音频配置 (为Week 4准备)
#[tauri::command]
pub async fn update_audio_config(
    event_manager: State<'_, EventManagerState>,
    config: AudioConfig,
) -> Result<(), String> {
    let manager_guard = event_manager.read().await;
    
    if let Some(manager) = manager_guard.as_ref() {
        let audio_event = AudioEvent::ConfigChanged { config };
        manager.emit_audio_event(audio_event, Some(EventPriority::Normal)).await
    } else {
        Err("事件管理器未初始化".to_string())
    }
}

/// 获取详细性能统计
#[tauri::command]
pub async fn get_detailed_performance_stats(
    performance_monitor: State<'_, PerformanceMonitorState>,
) -> Result<serde_json::Value, String> {
    let monitor_guard = performance_monitor.read().await;
    
    if let Some(monitor) = monitor_guard.as_ref() {
        let stats = monitor.get_all_stats().await;
        Ok(serde_json::to_value(stats).unwrap())
    } else {
        Err("性能监控器未初始化".to_string())
    }
}

/// 重置性能监控数据
#[tauri::command]
pub async fn reset_performance_data(
    performance_monitor: State<'_, PerformanceMonitorState>,
) -> Result<(), String> {
    let monitor_guard = performance_monitor.read().await;
    
    if let Some(_monitor) = monitor_guard.as_ref() {
        // 重新创建性能监控器实例
        drop(monitor_guard);
        let mut new_monitor_guard = performance_monitor.write().await;
        *new_monitor_guard = Some(PerformanceMonitor::new(None));
        Ok(())
    } else {
        Err("性能监控器未初始化".to_string())
    }
}

/// 触发系统诊断
#[tauri::command]
pub async fn trigger_system_diagnostics(
    performance_monitor: State<'_, PerformanceMonitorState>,
    event_manager: State<'_, EventManagerState>,
) -> Result<serde_json::Value, String> {
    let monitor_guard = performance_monitor.read().await;
    let event_guard = event_manager.read().await;
    
    if let (Some(monitor), Some(event_mgr)) = (monitor_guard.as_ref(), event_guard.as_ref()) {
        let health = monitor.get_system_health().await;
        let alerts = monitor.get_active_alerts().await;
        let queue_status = event_mgr.get_queue_status().await;
        let event_stats = event_mgr.get_stats().await;
        
        let diagnostics = serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "system_health": health,
            "active_alerts": alerts,
            "event_queue_status": {
                "current_size": queue_status.0,
                "max_size": queue_status.1,
                "utilization": (queue_status.0 as f64 / queue_status.1 as f64) * 100.0
            },
            "event_statistics": event_stats,
            "recommendations": generate_recommendations(&health, &alerts, queue_status.0, queue_status.1)
        });
        
        Ok(diagnostics)
    } else {
        Err("系统组件未初始化".to_string())
    }
}

/// 生成系统优化建议
fn generate_recommendations(
    health: &SystemHealth,
    alerts: &[PerformanceAlert],
    queue_size: usize,
    max_queue_size: usize,
) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    // 基于健康状态的建议
    match health.overall_status {
        crate::services::events::HealthStatus::Critical => {
            recommendations.push("系统处于严重状态，建议立即重启应用".to_string());
        }
        crate::services::events::HealthStatus::Warning => {
            recommendations.push("系统性能有所下降，建议检查后台任务".to_string());
        }
        _ => {}
    }
    
    // 基于警报的建议
    for alert in alerts {
        match alert.alert_type {
            crate::services::events::AlertType::HighLatency => {
                recommendations.push("检测到高延迟，建议减少并发操作".to_string());
            }
            crate::services::events::AlertType::EventBacklog => {
                recommendations.push("事件处理积压，建议检查事件处理器性能".to_string());
            }
            crate::services::events::AlertType::MemoryLeak => {
                recommendations.push("可能存在内存泄漏，建议重启应用".to_string());
            }
            _ => {}
        }
    }
    
    // 基于队列使用率的建议
    let utilization = (queue_size as f64 / max_queue_size as f64) * 100.0;
    if utilization > 80.0 {
        recommendations.push("事件队列使用率过高，建议增加处理器数量".to_string());
    }
    
    if recommendations.is_empty() {
        recommendations.push("系统运行正常，无特殊建议".to_string());
    }
    
    recommendations
} 