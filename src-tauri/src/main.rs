// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cli::process_args;
use config::{default_config, init_config};
use log::print_info;
use proxy::set_redirect_server;
use tauri::async_runtime::block_on;

mod certificate;
mod cli;
mod config;
mod lang;
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
    let crt_path = maybe_generate_ca();
    certificate::install_ca_files(crt_path.join("cert.crt"), Some(window));
}

pub fn maybe_generate_ca() -> std::path::PathBuf {
    let crt_path = certificate::cert_path();

    // If the cert.crt doesn't exist, generate it
    if !crt_path.join("cert.crt").exists() {
        print_info("Generating CA files...".to_string());
        certificate::generate_ca_files(certificate::cert_path());
    }

    crt_path
}

#[tauri::command]
async fn open_log_window(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let window =
            tauri::WindowBuilder::new(&app, "local", tauri::WindowUrl::App("index.html".into()))
                .build()
                .unwrap();

        window.set_title("reMITM - Logs").unwrap();

        window.eval("window.location.pathname = '/logs'").unwrap();
    });
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
    set_redirect_server(
        config
            .redirect_to
            .unwrap_or_else(|| default_config().redirect_to.unwrap()),
    );

    // Parse args with CLAP
    block_on(process_args());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            connect,
            disconnect,
            get_platform,
            get_hash,
            install_ca_command,
            open_log_window,
            config::get_config,
            config::write_config,
            lang::get_language,
            lang::language_list,
            proxy::set_redirect_server,
            tools::open_shell,
            tools::read_as_text,
        ])
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                disconnect();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn connect(app: tauri::AppHandle) {
    print_info("Connecting...".to_string());

    proxy::connect_to_proxy();
    proxy::create_proxy(Some(app)).await;
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
