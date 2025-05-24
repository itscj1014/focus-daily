// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 模块声明
mod models;
mod database;

// 导入必要的模块
use database::get_sql_plugin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(get_sql_plugin().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
