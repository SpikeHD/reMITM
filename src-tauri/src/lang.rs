use include_dir::{Dir, include_dir};
use serde_json::Value;

use crate::{
  config::get_config,
  log::{print_info, print_warning},
};

static LANG_DIR: Dir<'_> = include_dir!("lang");
pub static mut LANG: Option<String> = None;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Lang {
  pub name: String,
  pub filename: String,
}

#[tauri::command]
pub fn get_language(
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

      if let Some(lang_file) = LANG_DIR.get_file(&format!("{}", language)) {
        print_info(format!("Loaded language file: {}", language));
        
        let contents = lang_file.contents_utf8().unwrap_or_else(|| {
          print_warning(format!("Failed to read language file: {}", language));
          "{}"
        });

        // Cache the file
        print_info("Reloading language cache".to_string());
        LANG = Some(contents.to_string());

        return contents.to_string();
      } else {
        print_warning(format!("Failed to load language file: {}", language));
      }

      return String::from("{}");
    }

    LANG.clone().unwrap()
  }
}

#[tauri::command]
pub fn language_list() -> Vec<Lang> {
  // Read the "lang" dir and return the list
  let mut langs: Vec<Lang> = vec![];
  let files = LANG_DIR.files();

  // Read each file and create a lang object from the "language" json key within it
  for file in files {
    if !file.path().to_str().unwrap_or("").ends_with(".json") {
      continue;
    }
    
    let contents = file.contents_utf8().unwrap_or_else(|| {
      print_warning(format!("Failed to read language file: {}", file.path().to_str().unwrap_or("")));
      "{}"
    });
    let json: Value = serde_json::from_str(&contents).unwrap_or_else(|_| {
      print_warning(format!("Failed to parse language file: {}", file.path().to_str().unwrap_or("")));
      serde_json::from_str("{}").unwrap()
    });

    let filename = file.path().file_name().unwrap();

    let lang = Lang {
      name: json["language"].to_string(),
      filename: String::from(filename.to_str().unwrap()),
    };

    langs.push(lang);
  }

  langs
}
