// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::init_config;

mod config;
mod proxy;

/**
 * Ensures config path and file exists
 */
pub fn init() {
  init_config();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      config::get_config,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
