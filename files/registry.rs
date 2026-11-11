//! Windows registry read helpers for Ia Writer Pro.

pub const APP_KEY: &str = "Software\\Ia\\Ia Writer Pro";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Ia Writer Pro";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Ia Writer Pro") },
        RegValue { name: "Version",     value: String::from("v1.7.5") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Ia Writer Pro") },
        RegValue { name: "Publisher",       value: String::from("Ia") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Ia Writer Pro\\uninstall.exe\"") },
    ]
}
