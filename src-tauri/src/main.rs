// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use config::init_config;

mod config;
mod proxy;
mod certificate;

/**
 * Ensures config path and file exists
 */
pub fn init() {
  init_config();

  let crt_path = certificate::cert_path();

  // If the cert.crt doesn't exist, generate it
  if !crt_path.join("cert.crt").exists() {
    certificate::generate_ca_files(certificate::cert_path());
  }

  certificate::install_ca_files(crt_path.join("cert.crt"));
}

fn main() {
  init();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      config::get_config,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
