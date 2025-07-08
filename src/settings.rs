use serde_json::{Value, json};
use std::io::{BufReader, Write};
use std::fs::OpenOptions;

pub const DEFAULT_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

pub fn change_download_manifest(manifest: &str) {
    let settings_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("settings.json")
        .unwrap();

    let default_settings = json!({
        "manifest": DEFAULT_MANIFEST,
        "folder": format!(
                            r#"{home}{sep}servers{sep}"#,
                            sep = std::path::MAIN_SEPARATOR_STR,
                            home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                        )
                        .as_str(),
    });

    let settings_json = if settings_file.metadata().unwrap().len() == 0 {
        default_settings
    } else {
        let reader = BufReader::new(&settings_file);
        serde_json::from_reader(reader).unwrap()
    };

    let new_settings = json!({
        "manifest": manifest,
        "folder": settings_json["folder"]
    });

    write_settings(new_settings);
}

pub fn change_server_folder(folder: &str) {
    let settings_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("settings.json")
        .unwrap();

    let default_settings = json!({
        "manifest": DEFAULT_MANIFEST,
        "folder": format!(
                            r#"{home}{sep}servers{sep}"#,
                            sep = std::path::MAIN_SEPARATOR_STR,
                            home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                        )
                        .as_str(),
    });

    let settings_json = if settings_file.metadata().unwrap().len() == 0 {
        default_settings
    } else {
        let reader = BufReader::new(&settings_file);
        serde_json::from_reader(reader).unwrap()
    };

    let new_settings = json!({
        "manifest": settings_json["manifest"],
        "folder": folder
    });
    write_settings(new_settings);
}

pub fn reset_settings() {
    let default_settings = json!({
        "manifest": DEFAULT_MANIFEST,
        "folder": format!(
                            r#"{home}{sep}servers{sep}"#,
                            sep = std::path::MAIN_SEPARATOR_STR,
                            home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                        )
                        .as_str(),
    });

    write_settings(default_settings);
}

fn write_settings(settings: Value) {
    let mut settings_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("settings.json")
        .unwrap();

    settings_file
        .write_all(settings.to_string().as_bytes())
        .unwrap();
    settings_file.flush().unwrap();
}

pub fn get_manifest_setting() -> String {
    let settings_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("settings.json")
        .unwrap();

    let default_settings = json!({
        "manifest": DEFAULT_MANIFEST,
        "folder": format!(
                            r#"{home}{sep}servers{sep}"#,
                            sep = std::path::MAIN_SEPARATOR_STR,
                            home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                        )
                        .as_str(),
    });

    let settings_json = if settings_file.metadata().unwrap().len() == 0 {
        default_settings
    } else {
        let reader = BufReader::new(&settings_file);
        serde_json::from_reader(reader).unwrap()
    };

    return settings_json["manifest"].to_string().replace('"', "");
}

pub fn get_folder_setting() -> String {
    let settings_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("settings.json")
        .unwrap();

    let default_settings = json!({
        "manifest": DEFAULT_MANIFEST,
        "folder": format!(
                            r#"{home}{sep}servers{sep}"#,
                            sep = std::path::MAIN_SEPARATOR_STR,
                            home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                        )
                        .as_str(),
    });

    let settings_json = if settings_file.metadata().unwrap().len() == 0 {
        default_settings
    } else {
        let reader = BufReader::new(&settings_file);
        serde_json::from_reader(reader).unwrap()
    };

    return settings_json["folder"].to_string().replace('"', "");
}
