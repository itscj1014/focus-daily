use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::RwLock;

use crate::models::UserSettings;
use crate::services::timer::{TimerManager, TimerState};
use crate::services::timer::timer_manager::CycleState;

/// 全局定时器管理器状态
pub type TimerManagerState = Arc<RwLock<Option<TimerManager>>>;

/// 初始化定时器管理器
#[tauri::command]
pub async fn init_timer_manager(
    app_handle: AppHandle,
    timer_manager: State<'_, TimerManagerState>,
) -> Result<(), String> {
    // 获取默认用户设置
    let default_settings = UserSettings::default();
    
    // 创建定时器管理器
    let manager = TimerManager::new(app_handle, default_settings);
    
    // 存储到全局状态
    let mut manager_guard = timer_manager.write().await;
    *manager_guard = Some(manager);
    
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