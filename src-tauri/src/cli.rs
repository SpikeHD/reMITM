use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::{
  certificate::install_ca_files,
  config::{get_config, write_config}, disconnect,
  log::print_info,
  maybe_generate_ca,
  proxy::set_redirect_server,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// Whether to disable the GUI
  #[arg(short, long, default_value = "false")]
  no_gui: bool,

  /// Set what site to redirect to
  #[arg(short = 't', long, default_value = "")]
  redirect_to: String,

  /// Set what port to listen on
  #[arg(short, long, default_value = "8080")]
  port: u16,

  /// Whether to run the command that installs the CA certificate
  #[arg(short, long, default_value = "false")]
  install_ca: bool,

  /// File that contains URIs to redirect
  #[arg(short, long)]
  file: Option<String>,

  /// Comma-seperated list of URIs to redirect
  #[arg(short, long)]
  redirect: Option<String>,
}

pub async fn process_args() {
  let args = Args::parse();

  if !args.no_gui {
    print_info("Starting with GUI".to_string());
    return;
  }

  print_info("Starting without GUI".to_string());

  if args.install_ca {
    print_info("Installing CA certificate...".to_string());

    let crt_path = maybe_generate_ca();
    install_ca_files(crt_path, None);
  }

  if args.port != 8080 {
    let mut config = get_config();

    config.proxy_port = Some(args.port);

    // Save the config
    write_config(config);

    print_info(format!("Proxy port set to custom: {}", args.port));
  }

  if !args.redirect_to.is_empty() {
    print_info(format!("Redirecting to: {}", args.redirect_to));
    set_redirect_server(args.redirect_to);
  }

  if args.file.is_some() {
    let file = args.file.unwrap();

    print_info(format!("Reading file: {}", file));

    let file_contents = std::fs::read_to_string(file).unwrap_or(String::new());

    if file_contents.is_empty() {
      print_info("File is empty".to_string());
    } else {
      let lines = file_contents.split('\n');
      let collected: Vec<String> = lines.map(|s| s.to_string()).collect();

      print_info(format!("Redirecting {} URIs", &collected.len()));

      let mut config = get_config();

      config.urls_to_redirect = Some(collected);

      // Save the config
      write_config(config);
    }
  }

  // This is processed after the file because it should take precedence
  if args.redirect.is_some() {
    let redirect = args.redirect.unwrap();
    let lines = redirect.split(',');

    print_info(format!("Redirecting the following URIs: {}", redirect));

    let mut config = get_config();

    config.urls_to_redirect = Some(lines.map(|s| s.to_string()).collect());

    // Save the config
    write_config(config);
  }

  print_info("Connecting...".to_string());

  crate::proxy::connect_to_proxy();

  crate::proxy::create_proxy(None).await;

  // Let this thread sleep until ctrl+c is pressed
  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();

  ctrlc::set_handler(move || {
    r.store(false, Ordering::Relaxed);
  })
  .expect("Error setting Ctrl-C handler");

  while running.load(Ordering::Relaxed) {
    std::thread::sleep(Duration::from_millis(100));
  }

  disconnect();
  std::process::exit(0);
}
