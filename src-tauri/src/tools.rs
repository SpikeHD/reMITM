

use crate::config;

// While these are all identical right now, it's possible they may need specific modifications in the future.
// If that ends up not being the case, I'll remove 'em.
#[cfg(target_os = "windows")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();

  println!("Starting {}", terminal);

  match open::that(
    terminal,
  ) {
    Ok(_) => (),
    Err(e) => println!("Failed to open terminal: {}", e),
  };
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();
  let proxy_string = format!("127.0.0.1:{}", config.proxy_port.unwrap_or_else(|| config::default_config().proxy_port.unwrap()));

  println!("Starting {}", terminal);

  if config.use_env_variables.unwrap_or_else(|| config::default_config().use_env_variables.unwrap()) {
    // Open with HTTP_PROXY and HTTPS_PROXY set
    match open::that(format!(
      // Some programs are dumb and only look at the lowercase version of the env variables
      "{} -e \"export HTTP_PROXY={} HTTPS_PROXY={} http_proxy={} https_proxy={}; {}\"",
      terminal,
      proxy_string,
      proxy_string,
      proxy_string,
      proxy_string,
      terminal
    )) {
      Ok(_) => (),
      Err(e) => println!("Failed to open terminal: {}", e),
    };
  } else {
    // Open without HTTP_PROXY and HTTPS_PROXY set
    match open::that(format!(
      "{} -e \"{}\"",
      terminal,
      terminal
    )) {
      Ok(_) => (),
      Err(e) => println!("Failed to open terminal: {}", e),
    };
  }
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn open_shell() {
  let config = config::get_config();
  let terminal = config.terminal.unwrap();
  let proxy_string = format!("127.0.0.1:{}", config.proxy_port.unwrap_or_else(|| config::default_config().proxy_port.unwrap()));

  println!("Starting {}", terminal);

  if config.use_env_variables.unwrap_or_else(|| config::default_config().use_env_variables.unwrap()) {
    // Open with HTTP_PROXY and HTTPS_PROXY set
    match open::that(format!(
      // Some programs are dumb and only look at the lowercase version of the env variables
      "{} -e \"export HTTP_PROXY={} HTTPS_PROXY={} http_proxy={} https_proxy={}; {}\"",
      terminal,
      proxy_string,
      proxy_string,
      proxy_string,
      proxy_string,
      terminal
    )) {
      Ok(_) => (),
      Err(e) => println!("Failed to open terminal: {}", e),
    };
  } else {
    // Open without HTTP_PROXY and HTTPS_PROXY set
    match open::that(format!(
      "{} -e \"{}\"",
      terminal,
      terminal
    )) {
      Ok(_) => (),
      Err(e) => println!("Failed to open terminal: {}", e),
    };
  }
}
