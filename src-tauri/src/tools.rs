use std::process::Command;

use crate::config;

// While these are all identical right now, it's possible they may need specific modifications in the future.
// If that ends up not being the case, I'll remove 'em.
#[cfg(target_os = "windows")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();

  let set_proxy = Command::new(terminal)
    .output()
    .expect("failed to execute process");
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();

  let set_proxy = Command::new(terminal)
    .output()
    .expect("failed to execute process");
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();

  let set_proxy = Command::new(terminal)
    .output()
    .expect("failed to execute process");
}
