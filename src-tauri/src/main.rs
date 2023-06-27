// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::init_config;

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

/**
 * Get platform name. Used for selectively greying-out or accepting certain configurations
 */
pub fn get_platform() -> String {
  #[cfg(target_os = "windows")]
  return "windows".to_string();

  #[cfg(target_os = "linux")]
  return "linux".to_string();

  #[cfg(target_os = "macos")]
  return "macos".to_string();
}

fn main() {
  // If we are in debug, don't reopen as admin/root
  if !is_elevated::is_elevated() && !cfg!(debug_assertions) {
    system::reopen_as_admin();
  }

  init();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      connect,
      disconnect,
      config::get_config,
      config::write_config,
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
