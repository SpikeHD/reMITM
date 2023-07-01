// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::{init_config, default_config};
use log::print_info;
use proxy::{set_redirect_server};

mod certificate;
mod config;
mod log;
mod matcher;
mod proxy;
mod system;
mod tools;

/**
 * Ensures config path and file exists
 */
pub fn init() {
  // Create data_dir/reMITM if it doesn't exist
  let mut data_dir = dirs::data_dir().unwrap();
  data_dir.push("reMITM");
  if !data_dir.exists() {
    std::fs::create_dir(&data_dir).unwrap();
  }

  let pretty_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

  print_info(format!("== Initializing {} ==", pretty_date));

  init_config();

  let crt_path = certificate::cert_path();

  // If the cert.crt doesn't exist, generate it
  if !crt_path.join("cert.crt").exists() {
    print_info("Generating CA files...".to_string());
    certificate::generate_ca_files(certificate::cert_path());
  }
}

#[tauri::command]
fn install_ca_command(window: tauri::Window) {
  let crt_path = certificate::cert_path();

  // If the cert.crt doesn't exist, generate it
  if !crt_path.join("cert.crt").exists() {
    print_info("Generating CA files...".to_string());
    certificate::generate_ca_files(certificate::cert_path());
  }

  certificate::install_ca_files(crt_path.join("cert.crt"), Some(window));
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
      install_ca_command,
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
  print_info("Connecting...".to_string());

  proxy::connect_to_proxy();
  proxy::create_proxy().await;
}

#[tauri::command]
fn disconnect() {
  print_info("Disconnecting...".to_string());
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