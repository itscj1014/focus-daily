// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 模块声明
mod models;
mod database;
mod services;

// 导入必要的模块
use database::get_sql_plugin;
use services::{
    TimerManagerState,
    init_timer_manager, get_timer_state, get_cycle_state,
    start_focus_session, start_long_break_session, start_micro_break_session,
    pause_timer, resume_timer, reset_timer, skip_micro_break,
    get_today_stats, update_timer_settings,
    EventManagerState, PerformanceMonitorState,
    get_event_stats, get_event_history, get_event_queue_status, cleanup_expired_events,
    get_performance_report, get_system_health, get_active_alerts, resolve_performance_alert,
    play_audio_event, update_audio_config, get_detailed_performance_stats,
    reset_performance_data, trigger_system_diagnostics,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建全局状态管理器
    let timer_manager_state: TimerManagerState = Arc::new(RwLock::new(None));
    let event_manager_state: EventManagerState = Arc::new(RwLock::new(None));
    let performance_monitor_state: PerformanceMonitorState = Arc::new(RwLock::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(get_sql_plugin().build())
        // 注册所有状态管理器
        .manage(timer_manager_state)
        .manage(event_manager_state)
        .manage(performance_monitor_state)
        .invoke_handler(tauri::generate_handler![
            // 原有的定时器命令
            init_timer_manager,
            get_timer_state,
            get_cycle_state,
            start_focus_session,
            start_long_break_session,
            start_micro_break_session,
            pause_timer,
            resume_timer,
            reset_timer,
            skip_micro_break,
            get_today_stats,
            update_timer_settings,
            
            // Day 4 新增的增强事件系统命令
            get_event_stats,
            get_event_history,
            get_event_queue_status,
            cleanup_expired_events,
            
            // Day 4 新增的性能监控命令
            get_performance_report,
            get_system_health,
            get_active_alerts,
            resolve_performance_alert,
            get_detailed_performance_stats,
            reset_performance_data,
            trigger_system_diagnostics,
            
            // Day 4 新增的音频事件命令 (为Week 4准备)
            play_audio_event,
            update_audio_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
