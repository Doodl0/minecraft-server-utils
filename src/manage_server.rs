use std::io::Write;

pub struct ServerInstance {
    name: String,
    folder: String,
    version: String,
    version_type: VersionType,
}

#[derive(Copy, Clone)]
pub enum VersionType {
    Release,
    Snapshot,
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
}
