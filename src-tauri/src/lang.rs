use std::{path::PathBuf, io::Read};

use serde_json::json;

use crate::config::get_config;

pub static mut LANG: Option<String> = None;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Lang {
  pub name: String,
  pub filename: String,
}

#[tauri::command]
pub fn get_language(app: tauri::AppHandle, force_reload: Option<bool>) -> String {
  let config = get_config();
  let language = config.language.clone().unwrap_or_else(|| String::from("en"));

  unsafe {
    if LANG.is_none() || force_reload.unwrap_or(false) {
      // Load the file
      let lang_file = app.path_resolver().resolve_resource(PathBuf::from(format!("lang/{}.json", language))).unwrap();
      let mut file = std::fs::File::open(lang_file).unwrap();
      let mut contents = String::new();

      file.read_to_string(&mut contents).unwrap();

      // Cache the file
      LANG = Some(contents);

      return LANG.clone().unwrap();
    }

    return LANG.clone().unwrap();
  }
}

#[tauri::command]
pub fn language_list(app: tauri::AppHandle) -> Vec<Lang> {
  // Read the "lang" dir and return the list
  let mut langs: Vec<Lang> = vec![];
  let files = app.path_resolver().resolve_resource(PathBuf::from("lang/")).unwrap();
  let file_list = std::fs::read_dir(files.clone()).unwrap();

  // Read each file and create a lang object from the "language" json key within it
  for file in file_list {
    let file_entry = file.unwrap();
    let file_path = &file_entry.path();

    let mut file = std::fs::File::open(file_path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let filename = file_path.file_name().unwrap();

    let lang = Lang {
      name: json["language"].to_string(),
      filename: String::from(filename.to_str().unwrap()),
    };

    langs.push(lang);
  }

  langs
}