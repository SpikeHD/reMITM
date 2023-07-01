use crate::config;
use crate::log::{print_info, print_error};

// While these are all identical right now, it's possible they may need specific modifications in the future.
// If that ends up not being the case, I'll remove 'em.
#[cfg(target_os = "windows")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();

  print_info(format!("Starting {}", terminal));

  match open::that(
    terminal,
  ) {
    Ok(_) => (),
    Err(e) => print_error(format!("Failed to open terminal: {}", e)),
  };
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();
  let proxy_string = format!("127.0.0.1:{}", config.proxy_port.unwrap_or_else(|| config::default_config().proxy_port.unwrap()));

  print_info(format!("Starting {}", terminal));
  print_info(format!("Set env variables? {}", config.use_env_variables.unwrap_or_else(|| config::default_config().use_env_variables.unwrap())));

  if config.use_env_variables.unwrap_or_else(|| config::default_config().use_env_variables.unwrap()) {
    // Open with HTTP_PROXY and HTTPS_PROXY set
    match open::with(
      format!(
        "{} -e \"export HTTP_PROXY={} HTTPS_PROXY={} http_proxy={} https_proxy={}\"",
        terminal,
        proxy_string,
        proxy_string,
        proxy_string,
        proxy_string
      ),
      terminal
    ) {
      Ok(_) => (),
      Err(e) => print_error(format!("Failed to open terminal: {}", e)),
    };
  } else {
    // Open without HTTP_PROXY and HTTPS_PROXY set
    match open::with(
      format!(
        "-e \"{}\"",
        terminal
      ),
      terminal
    ) {
      Ok(_) => (),
      Err(e) => print_error(format!("Failed to open terminal: {}", e)),
    };
  }
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();
  let proxy_string = format!("127.0.0.1:{}", config.proxy_port.unwrap_or_else(|| config::default_config().proxy_port.unwrap()));

  print_info(format!("Starting {}", terminal));
  print_info(format!("Set env variables? {}", config.use_env_variables.unwrap_or_else(|| config::default_config().use_env_variables.unwrap())));

  if config.use_env_variables.unwrap_or_else(|| config::default_config().use_env_variables.unwrap()) {
    // Open with HTTP_PROXY and HTTPS_PROXY set
    match open::with(
      format!(
        "{} -e \"export HTTP_PROXY={} HTTPS_PROXY={} http_proxy={} https_proxy={}\"",
        terminal,
        proxy_string,
        proxy_string,
        proxy_string,
        proxy_string
      ),
      terminal
    ) {
      Ok(_) => (),
      Err(e) => print_error(format!("Failed to open terminal: {}", e)),
    };
  } else {
    // Open without HTTP_PROXY and HTTPS_PROXY set
    match open::with(
      format!(
        "-e \"{}\"",
        terminal
      ),
      terminal
    ) {
      Ok(_) => (),
      Err(e) => print_error(format!("Failed to open terminal: {}", e)),
    };
  }
}
