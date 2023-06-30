// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::{init_config, default_config};
use proxy::{set_redirect_server};

mod certificate;
mod config;
mod matcher;
mod proxy;
mod system;
mod tools;

/**
 * Ensures config path and file exists
 */
pub fn init() {
  init_config();

  let crt_path = certificate::cert_path();

  // If the cert.crt doesn't exist, generate it
  if !crt_path.join("cert.crt").exists() {
    println!("Generating CA files...");
    certificate::generate_ca_files(certificate::cert_path());
  }

  certificate::install_ca_files(crt_path.join("cert.crt"));
}

fn main() {
  // If we are in debug, don't reopen as admin/root
  #[cfg(target_os = "windows")]
  if !is_elevated::is_elevated() && !cfg!(debug_assertions) {
    system::reopen_as_admin();
  }

  init();

  let config = config::get_config();
  // set redirect to via config
  set_redirect_server(config.redirect_to.unwrap_or_else(|| default_config().redirect_to.unwrap()));

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      connect,
      disconnect,
      get_platform,
      get_hash,
      config::get_config,
      config::write_config,
      proxy::set_redirect_server,
      tools::open_shell,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn connect() {
  println!("Connecting...");

  proxy::connect_to_proxy();
  proxy::create_proxy().await;
}

#[tauri::command]
fn disconnect() {
  println!("Disconnecting...");
  proxy::disconnect_from_proxy();
}

/**
 * Get platform name. Used for selectively greying-out or accepting certain configurations
 */
#[tauri::command]
fn get_platform() -> String {
  #[cfg(target_os = "windows")]
  return "windows".to_string();

  #[cfg(target_os = "linux")]
  return "linux".to_string();

  #[cfg(target_os = "macos")]
  return "macos".to_string();
}

#[tauri::command]
fn get_hash() -> String {
  option_env!("HASH").unwrap_or("UNKNOWN").to_string()
}