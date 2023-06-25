use serde::de::value::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
  pub launch_at_startup: bool,
  pub proxy_port: u16,
  pub urls_to_redirect: Vec<String>,
  pub redirect_to: String,
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
    Config {
      launch_at_startup: false,
      proxy_port: 8111,
      urls_to_redirect: vec![],
      redirect_to: "http://localhost:3000".to_string(),
    }
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
    let config = Config {
      launch_at_startup: false,
      proxy_port: 8111,
      urls_to_redirect: vec![],
      redirect_to: "http://localhost:3000".to_string(),
    };

    let config_json = serde_json::to_string(&config).unwrap();
    fs::write(path, config_json).unwrap();
  }
}