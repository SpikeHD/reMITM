use colored::Colorize;
use dirs::data_dir;
use once_cell::sync::Lazy;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;

// Open a global filestream to write to
static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| {
  Mutex::new(
    OpenOptions::new()
      .append(true)
      .create(true)
      .open(data_dir().unwrap().join("reMITM").join("log.txt"))
      .unwrap(),
  )
});

pub fn append_logfile(message: String) {
  let pretty_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
  let mut file = LOG_FILE.lock().unwrap();
  file.write_all(format!("{} | {}\n", pretty_date, message).as_bytes()).unwrap();
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

pub fn _print_warning(message: String) {
  print_pretty("[WARNING]".yellow().to_string(), message.clone());

  // Write to log file
  append_logfile(format!("[WARNING] {}", message));
}

pub fn print_info(message: String) {
  print_pretty("[INFO]".blue().to_string(), message.clone());

  // Write to log file
  append_logfile(format!("[INFO] {}", message));
}
