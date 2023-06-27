use serde::de::value::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
  pub launch_at_startup: Option<bool>,
  pub proxy_port: Option<u16>,
  pub urls_to_redirect: Option<Vec<String>>,
  pub redirect_to: Option<String>,
  pub log_requests: Option<bool>,
  pub terminal: Option<String>,
  pub modify_gsettings: Option<bool>,
  pub use_env_variables: Option<bool>,
}

pub fn default_config() -> Config {
  Config {
    launch_at_startup: Some(false),
    proxy_port: Some(8111),
    urls_to_redirect: Some(vec![]),
    redirect_to: Some("http://localhost:3000".to_string()),
    log_requests: Some(false),

    // C:\Windows\System32\cmd.exe for windows, gnome-terminal for linux, Terminal.app for mac
    #[cfg(target_os = "windows")]
    terminal: Some("C:\\Windows\\System32\\cmd.exe".to_string()),

    #[cfg(target_os = "linux")]
    terminal: Some("gnome-terminal".to_string()),

    #[cfg(target_os = "macos")]
    terminal: Some("Terminal.app".to_string()),

    // On linux you may want to enable using http(s)_proxy env variables and/or gsettings
    #[cfg(target_os = "linux")]
    modify_gsettings: Some(true),
    #[cfg(target_os = "linux")]
    use_env_variables: Some(true),

    // Other platforms don't use/care about those
    #[cfg(not(target_os = "linux"))]
    modify_gsettings: Some(false),
    #[cfg(not(target_os = "linux"))]
    use_env_variables: Some(false),
  }
}

pub fn config_path() -> PathBuf {
  let mut path = dirs::config_dir().unwrap();
  path.push("reMITM");
  path.push("config.json");
  path
}

#[tauri::command]
pub fn get_config() -> Config {
  let path = config_path();

  if path.exists() {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(&contents).unwrap()
  } else {
   default_config()
  }
}

pub fn write_config(config: Config) {
  let path = config_path();
  let config_json = serde_json::to_string(&config).unwrap();
  fs::write(path, config_json).unwrap();
}

pub fn init_config() {
  let path = config_path();

  // Ensure config_dir/reMITM exists
  if !path.parent().unwrap().exists() {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
  }

  if !path.exists() {
    let config = default_config();

    let config_json = serde_json::to_string(&config).unwrap();
    fs::write(path, config_json).unwrap();
  }
}