// Prevents additional console window on Windows in both debug and release, DO NOT REMOVE!!
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use tauri::{Builder, Manager};
use std::sync::Mutex;

// 全局狀態管理
static APP_STATE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);

/// Tauri 應用程式狀態
#[derive(Default)]
struct AppState {
    web_url: String,
    desktop_mode: bool,
}

/// 獲取 Web URL
#[tauri::command]
fn get_web_url(state: tauri::State<AppState>) -> String {
    state.web_url.clone()
}

/// 設置 Web URL
#[tauri::command]
fn set_web_url(url: String, _state: tauri::State<AppState>) {
    println!("設置 Web URL: {}", url);
}

/// 檢查是否為桌面模式
#[tauri::command]
fn is_desktop_mode(state: tauri::State<AppState>) -> bool {
    state.desktop_mode
}

/// 設置桌面模式
#[tauri::command]
fn set_desktop_mode(enabled: bool, _state: tauri::State<AppState>) {
    println!("設置桌面模式: {}", enabled);
}

fn main() {
    // 初始化日誌
    env_logger::init();

    println!("正在啟動 MCP Feedback Enhanced 桌面應用程式...");

    // 創建 Tauri 應用程式
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .setup(|app| {
            // 儲存應用程式句柄到全局狀態
            {
                let mut state = APP_STATE.lock().unwrap();
                *state = Some(app.handle().clone());
            }

            // 獲取主視窗，先統一設定標題，再根據環境變數決定是否導航
            if let Some(window) = app.get_webview_window("main") {
                let app_version = std::env::var("MCP_APP_VERSION")
                    .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string());
                let window_title = format!("MCP Feedback Enhanced Pro v{}", app_version);
                let _ = window.set_title(&window_title);

                // 檢查是否有 MCP_WEB_URL 環境變數
                if let Ok(web_url) = std::env::var("MCP_WEB_URL") {
                    println!("檢測到 Web URL: {}", web_url);
                    let _ = window.navigate(web_url.parse().unwrap());
                }
            }

            println!("Tauri 應用程式已初始化");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_web_url,
            set_web_url,
            is_desktop_mode,
            set_desktop_mode
        ])
        .run(tauri::generate_context!())
        .expect("運行 Tauri 應用程式時發生錯誤");
}
