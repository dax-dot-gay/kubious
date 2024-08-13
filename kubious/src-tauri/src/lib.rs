// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod api;
use std::{fs::{self, File}, io::{Read, Write}};

use api::{app_state::AppState, execute_command, ApiCommand, CommandHandler, CommandResult};
use tauri::{AppHandle, Manager};

mod compat;

#[tauri::command]
async fn execute_api_command(app_handle: AppHandle, command: ApiCommand) -> CommandResult {
    execute_command(app_handle, command).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let resolver = app.path();
            let config_root = resolver.app_config_dir().unwrap();
            if !config_root.exists() {
                fs::create_dir(config_root).expect("Unable to create config directory");
            }
            if !resolver.parse("$APPCONFIG/config.json").unwrap().exists() {
                let mut config_file = File::create(resolver.parse("$APPCONFIG/config.json").unwrap()).expect("Failed to create config.json");

                let new_state = AppState::new();
                let registered = tauri::async_runtime::block_on(async move {
                    new_state.register_default().await;
                    new_state
                });

                config_file.write_all(registered.to_json().expect("Failed to serialize new state").as_bytes()).expect("Failed to write to config.json");
            }

            let mut config_file = File::open(resolver.parse("$APPCONFIG/config.json").unwrap()).expect("Failed to open config.json");
            let mut contents = String::new();
            config_file.read_to_string(&mut contents).expect("Failed to read config.json");
            app.manage(AppState::from_json(contents.as_str()).expect("Failed to parse config"));

            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![execute_api_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
