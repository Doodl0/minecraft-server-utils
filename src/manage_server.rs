use std::{io::Write};

struct SeverInstance {
    name: String
}

pub fn create_server_directory(server_name: &str) {
    let _ = std::fs::create_dir_all(format!(
        r#"{home}{sep}servers{sep}{server_name}{sep}"#,
        sep = std::path::MAIN_SEPARATOR_STR,
        home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
    ));
}

pub fn agree_to_eula(server_name: &str) {
    let _ = std::fs::File::create(format!(
        r#"{home}{sep}servers{sep}{server_name}{sep}eula.txt"#,
        sep = std::path::MAIN_SEPARATOR_STR,
        home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
    )).unwrap().write_all(b"eula=true");
}