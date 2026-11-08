//! Windows registry read helpers for Serum Drum Rack.

pub const APP_KEY: &str = "Software\\Serum\\Serum Drum Rack";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Serum Drum Rack";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Serum Drum Rack") },
        RegValue { name: "Version",     value: String::from("v4.4.8") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Serum Drum Rack") },
        RegValue { name: "Publisher",       value: String::from("Serum") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Serum Drum Rack\\uninstall.exe\"") },
    ]
}
