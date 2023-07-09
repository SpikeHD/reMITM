use std::{io::Read, path::PathBuf};

use crate::{
  config::get_config,
  log::{print_info, print_warning},
};

pub static mut LANG: Option<String> = None;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Lang {
  pub name: String,
  pub filename: String,
}

#[tauri::command]
pub fn get_language(
  app: tauri::AppHandle,
  force_reload: Option<bool>,
  lang: Option<String>,
) -> String {
  let config = get_config();

  // Load from "lang" argument, or from the config otherwise. Default to english
  let language = lang.unwrap_or_else(|| config.language.unwrap_or_else(|| String::from("en")));

  unsafe {
    if LANG.is_none() || force_reload.unwrap_or(false) {
      // Load the file
      print_info(format!("Loading language file: {}", language));

      let lang_file = app
        .path_resolver()
        .resolve_resource(PathBuf::from(format!("lang/{}", language)))
        .unwrap();
      let mut file = std::fs::File::open(lang_file).unwrap_or_else(|_| {
        print_warning(format!(
          "Error loading language file: {}, falling back to english.",
          language
        ));

        // Fallback to english
        std::fs::File::open(
          app
            .path_resolver()
            .resolve_resource(PathBuf::from("lang/en.json"))
            .unwrap(),
        )
        .unwrap()
      });
      let mut contents = String::new();

      file.read_to_string(&mut contents).unwrap();

      // Cache the file
      if LANG.is_none() || force_reload.unwrap_or(false) {
        print_info("Reloading language cache".to_string());
        LANG = Some(contents.clone());
      }

      return contents;
    }

    LANG.clone().unwrap()
  }
}

#[tauri::command]
pub fn language_list(app: tauri::AppHandle) -> Vec<Lang> {
  // Read the "lang" dir and return the list
  let mut langs: Vec<Lang> = vec![];
  let files = app
    .path_resolver()
    .resolve_resource(PathBuf::from("lang/"))
    .unwrap();
  let file_list = std::fs::read_dir(files).unwrap();

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
