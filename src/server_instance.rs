use core::fmt;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, Write};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerInstance {
    pub name: String,
    pub folder: String,
    pub version: String,
    pub version_type: VersionType,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum VersionType {
    Release,
    Snapshot,
}

impl fmt::Display for VersionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ServerInstance {
    pub fn new(
        name: String,
        folder: String,
        version: String,
        version_type: VersionType,
    ) -> ServerInstance {
        ServerInstance {
            name,
            folder,
            version,
            version_type,
        }
    }

    pub fn agree_to_eula(&self) {
        let mut eula = std::fs::File::create(format!(
            r#"{folder}{sep}eula.txt"#,
            sep = std::path::MAIN_SEPARATOR_STR,
            folder = self.folder,
        ))
        .unwrap();
        let _ = eula.write_all(b"eula=true");
    }

    pub fn create_server_directory(&mut self, directory: Option<&str>) {
        // If user specified directory, set server folder there
        // Else, set server folder in home/servers
        match directory {
            Some(dir) => {
                self.folder = format!(
                    r#"{home}{sep}{name}{sep}"#,
                    sep = std::path::MAIN_SEPARATOR_STR,
                    home = dir,
                    name = self.name
                )
            }
            None => {
                self.folder = format!(
                    r#"{home}{sep}servers{sep}{name}{sep}"#,
                    sep = std::path::MAIN_SEPARATOR_STR,
                    home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                    name = self.name
                )
            }
        }

        // Create server folder
        let _ = std::fs::create_dir_all(&self.folder);
    }

    pub fn add_to_server_list(&mut self) {
        let self_owned = ServerInstance {
            name: self.name.to_owned(),
            folder: self.folder.to_owned(),
            version: self.version.to_owned(),
            version_type: self.version_type.to_owned(),
        };

        let server_list_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("server-list.json")
            .unwrap();

        let mut data: Vec<ServerInstance> = list_saved_servers(server_list_file);

        data.push(self_owned);

        let mut server_list_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("server-list.json")
            .unwrap();

        // Serialize and write json to file
        let serialized_json = serde_json::to_string_pretty(&data).unwrap();
        server_list_file
            .write_all(serialized_json.as_bytes())
            .unwrap();
        server_list_file.flush().unwrap();
    }

    pub fn delete(&mut self) {
        std::fs::remove_dir_all(self.folder.clone()).expect("Could not delete server folder");

        // Take data from file and rebuild without self 
        let server_list_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("server-list.json")
            .unwrap();
        let mut data: Vec<ServerInstance> = list_saved_servers(server_list_file);

        for (index, server) in data.clone().iter().enumerate() {
            if server.name == self.name {
                data.remove(index);
            }
        }

        let mut server_list_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("server-list.json")
            .unwrap();

        // Serialize and write json to file
        let serialized_json = serde_json::to_string_pretty(&data).unwrap();
        server_list_file
            .write_all(serialized_json.as_bytes())
            .unwrap();
        server_list_file.flush().unwrap();
}}

pub fn list_saved_servers(file: std::fs::File) -> Vec<ServerInstance> {
    // If file is empty, make an empty vec
    // Else, parse vec from json
    let data: Vec<ServerInstance> = if file.metadata().unwrap().len() == 0 {
        Vec::new()
    } else {
        let reader = BufReader::new(&file);
        serde_json::from_reader(reader).unwrap()
    };

    return data;
}
