use colored::Colorize;
use dirs::data_dir;
use once_cell::sync::Lazy;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;

// Open a global filestream to write tos
static mut LOG_FILE: Option<Lazy<Mutex<File>>> = None;

pub fn append_logfile(message: String) {
  unsafe {
    if LOG_FILE.is_none() {
      LOG_FILE = Some(Lazy::new(|| {
        Mutex::new(
          OpenOptions::new()
            .append(true)
            .create(true)
            .open(data_dir().unwrap().join("reMITM").join("log.txt"))
            .unwrap(),
        )
      }));
    }
  }

  let pretty_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

  unsafe {
    let file = LOG_FILE.as_ref().unwrap();
    let mut file = file.lock().unwrap();
    file
      .write_all(format!("{} | {}\n", pretty_date, message).as_bytes())
      .unwrap();
  }
}

pub fn print_pretty(kind: String, message: String) {
  let pretty_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
  println!("{} | {} {}", pretty_date, kind.bold(), message);
}

pub fn print_error(message: String) {
  print_pretty("[ERROR]".red().to_string(), message.clone());

  // Write to log file
  append_logfile(format!("[ERROR] {}", message));
}

pub fn print_warning(message: String) {
  print_pretty("[WARNING]".yellow().to_string(), message.clone());

  // Write to log file
  append_logfile(format!("[WARNING] {}", message));
}

pub fn print_info(message: String) {
  print_pretty("[INFO]".blue().to_string(), message.clone());

  // Write to log file
  append_logfile(format!("[INFO] {}", message));
}
