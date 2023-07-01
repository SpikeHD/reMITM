use std::fs;
use std::path::{PathBuf};
use std::process::Command;

use tauri::api::dialog::message;

use rcgen::*;

pub fn cert_path() -> PathBuf {
  let mut path = dirs::config_dir().unwrap();
  path.push("reMITM");
  path.push("ca");
  path
}

/*
 * Generates a private key and certificate used by the certificate authority.
 * Additionally installs the certificate and private key in the Root CA store.
 * Source: https://github.com/zu1k/good-mitm/raw/master/src/ca/gen.rs
 */
#[tauri::command]
pub fn generate_ca_files(cert_dir: PathBuf) {
  let mut params = CertificateParams::default();
  let mut details = DistinguishedName::new();

  // Set certificate details.
  details.push(DnType::CommonName, "reMITM");
  details.push(DnType::OrganizationName, "SpikeHD");
  details.push(DnType::CountryName, "US");
  details.push(DnType::LocalityName, "US");

  // Set details in the parameter.
  params.distinguished_name = details;
  // Set other properties.
  params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
  params.key_usages = vec![
    KeyUsagePurpose::DigitalSignature,
    KeyUsagePurpose::KeyCertSign,
    KeyUsagePurpose::CrlSign,
  ];

  // Create certificate.
  let cert = Certificate::from_params(params).unwrap();
  let cert_crt = cert.serialize_pem().unwrap();
  let private_key = cert.serialize_private_key_pem();

  // Make certificate directory.
  match fs::create_dir(&cert_dir) {
    Ok(_) => {}
    Err(e) => {
      println!("{}", e);
    }
  };

  // Write the certificate to a file.
  let cert_path = cert_dir.join("cert.crt");
  match fs::write(&cert_path, cert_crt) {
    Ok(_) => println!("Wrote certificate to {}", cert_path.to_str().unwrap()),
    Err(e) => println!(
      "Error writing certificate to {}: {}",
      cert_path.to_str().unwrap(),
      e
    ),
  }

  // Write the private key to a file.
  let private_key_path = cert_dir.join("private.key");
  match fs::write(&private_key_path, private_key) {
    Ok(_) => println!(
      "Wrote private key to {}",
      private_key_path.to_str().unwrap()
    ),
    Err(e) => println!(
      "Error writing private key to {}: {}",
      private_key_path.to_str().unwrap(),
      e
    ),
  }

  // (Linux only) chmod to let certutil read the file
  #[cfg(target_os = "linux")]
  {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&cert_path).unwrap().permissions();
    perms.set_mode(0o777);
    fs::set_permissions(&cert_path, perms).unwrap();
  }

  // Install certificate into the system's Root CA store.
  install_ca_files(cert_path.join("cert.crt"), None);
}

#[cfg(target_os = "windows")]
pub fn install_ca_files(path: PathBuf, app: Option<tauri::Window>) {
  // Check if cert already exists
  let cert_exists = Command::new("certutil")
    .arg("-store")
    .arg("-user")
    .arg("Root")
    .arg(&path)
    .output()
    .expect("failed to execute process");

  println!("{:?}", cert_exists);

  if !cert_exists.status.success() {
    // Install certificate
    let install_cert = Command::new("certutil")
      .arg("-addstore")
      .arg("-user")
      .arg("Root")
      .arg(&path)
      .output()
      .expect("failed to execute process");

    if install_cert.status.success() {
      println!("Installed certificate into Root CA store");
    } else {
      println!("Error installing certificate into Root CA store");
      println!("{:?}", install_cert);
      
      // This is a special case where the user should definitely be aware of what to do next
      if let Some(app) = app {
        message(
          Some(&app),
          "CertUtil Error",
          format!("There was an error installing the certificate: \n\n{}",std::str::from_utf8(&install_cert.stderr).unwrap_or_else(|_| "Unknown error"))
        );
      }
    }
  } else {
    println!("Certificate already exists in Root CA store");
  }
}

#[cfg(target_os = "linux")]
pub fn install_ca_files(path: PathBuf, app: Option<tauri::Window>) {
  use std::{process::Stdio, io::Write};

  // Check if cert already exists
  let cert_exists = Command::new("certutil")
    .arg("-d")
    // This is kinda weird but has to be done
    .arg(format!("sql:{}/.pki/nssdb", std::env::var("HOME").unwrap()))
    .arg("-L")
    .arg("-n")
    .arg("reMITM")
    .output()
    .expect("failed to execute process");

  if !cert_exists.status.success() {
    // Install certificate
    let mut install_cert = Command::new("certutil")
      .arg("-d")
      // Also weird but has to be done
      .arg(format!("sql:{}/.pki/nssdb", std::env::var("HOME").unwrap()))
      .arg("-A")
      .arg("-t")
      .arg("C,,")
      .arg("-n")
      .arg("reMITM")
      .arg("-a")
      .stdin(Stdio::piped())
      .spawn()
      .unwrap();

    if let Some(mut stdin) = install_cert.stdin.take() {
      stdin.write_all(&fs::read(path).unwrap()).unwrap();
    }

    let install_finish = install_cert.wait_with_output().unwrap();

    if install_finish.status.success() {
      println!("Installed certificate into Root CA store");
    } else {
      println!("Error installing certificate into Root CA store");
      println!("{:?}", install_finish);

      // This is a special case where the user should definitely be aware of what to do next
      if let Some(app) = app {
        message(
          Some(&app),
          "CertUtil Error",
          format!("There was an error installing the certificate: \n\n{}", std::str::from_utf8(&install_finish.stderr).unwrap_or("Unknown error"))
        );
      }
    }
  } else {
    println!("Certificate already exists in Root CA store");
  }
}

#[cfg(target_os = "macos")]
pub fn install_ca_files(path: PathBuf, app: Option<tauri::Window>) {
  // Check if cert already exists
  let cert_exists = Command::new("security")
    .arg("find-certificate")
    .arg("-c")
    .arg("reMITM")
    .arg("-a")
    .arg("-Z")
    .arg(&path)
    .output()
    .expect("failed to execute process");

  if !cert_exists.status.success() {
    // Install certificate
    let install_cert = Command::new("security")
      .arg("add-trusted-cert")
      .arg("-d")
      .arg("-r")
      .arg("trustRoot")
      .arg("-k")
      .arg("/Library/Keychains/System.keychain")
      .arg(&path)
      .output()
      .expect("failed to execute process");

    if install_cert.status.success() {
      println!("Installed certificate into Root CA store");
    } else {
      println!("Error installing certificate into Root CA store");
      println!("{:?}", install_cert);
      
      // This is a special case where the user should definitely be aware of what to do next
      if let Some(app) = app {
        message(
          Some(&app),
          "CertUtil Error",
          format!("There was an error installing the certificate: \n\n{}",std::str::from_utf8(&install_cert.stderr).unwrap_or_else(|_| "Unknown error"))
        );
      }
    }
  } else {
    println!("Certificate already exists in Root CA store");
  }
}