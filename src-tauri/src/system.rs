use std::process::{exit, Command};

#[cfg(windows)]
pub fn reopen_as_admin() {
    let install = std::env::current_exe().unwrap();

    Command::new("powershell.exe")
        .arg("powershell")
        .arg(format!(
            "-command \"&{{Start-Process -filepath '{}' -verb runas}}\"",
            install.to_str().unwrap()
        ))
        .spawn()
        .expect("Error starting exec as admin");

    exit(0);
}

#[cfg(target_os = "linux")]
pub fn _reopen_as_admin() {
    let install = std::env::current_exe().unwrap();

    Command::new("sudo")
        .arg(install.to_str().unwrap())
        .spawn()
        .expect("Error starting exec as admin");

    exit(0);
}

#[cfg(target_os = "macos")]
pub fn _reopen_as_admin() {
    let install = std::env::current_exe().unwrap();

    Command::new("sudo")
        .arg(install.to_str().unwrap())
        .spawn()
        .expect("Error starting exec as admin");

    exit(0);
}
