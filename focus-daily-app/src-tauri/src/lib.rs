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
    get_today_stats, update_timer_settings
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建全局定时器管理器状态
    let timer_manager_state: TimerManagerState = Arc::new(RwLock::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(get_sql_plugin().build())
        .manage(timer_manager_state)
        .invoke_handler(tauri::generate_handler![
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
            update_timer_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
