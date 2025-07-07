use crate::server_instance::VersionType;
use reqwest::blocking::Response;

const DEFAULT_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

// Get manifest of servers from mojang.com (or other manifests)
fn get_json_manifest(url: &str) -> serde_json::Value {
    // Send request to url
    let manifest:Response = reqwest::blocking::get(url).expect(format!("Couldn't recieve JSON manifest {}, please check your internet connection or the manifest URL.", url).as_str());

    // Convert .json file into serde_json::Value
    let manifest_json: serde_json::Value = serde_json::from_str(
        &manifest
            .text()
            .expect("Couldn't convert JSON manifest to text."),
    )
    .unwrap();

    return manifest_json;
}

// Retrieve letest release name from server
fn get_latest_release(url: &str) -> String {
    let manifest: serde_json::Value = get_json_manifest(url);
    return manifest["latest"]["release"].to_string().replace('"', "");
}

// Get the URL value of a version
fn get_version_url(version: &str, url: &str) -> Option<String> {
    let manifest: serde_json::Value = get_json_manifest(url);
    let versions: Option<&Vec<serde_json::Value>> = manifest["versions"].as_array();

    for id in versions.unwrap() {
        let ver: String = id["id"].to_string().replace('"', "");

        if version == ver {
            let url: String = id["url"].to_string().replace('"', "");
            return Some(url);
        }
    }

    return None;
}

// Download a server.jar, with optional version
fn download_server_jar(version: Option<&str>, filepath: &str, manifest: Option<&str>) {
    let url: &str;
    let ver: String;
    match manifest {
        Some(t) => url = t,
        None => url = DEFAULT_MANIFEST,
    }
    match version {
        Some(t) => ver = t.to_string(),
        None => ver = get_latest_release(url),
    }

    let version_url = get_version_url(&ver, url).unwrap().replace('"', "");
    let download_link = get_json_manifest(&version_url.as_str())["downloads"]["server"]["url"]
        .to_string()
        .replace('"', "");
    save_file(&download_link, filepath.to_string());
}

// Save a file from a download link
fn save_file(url: &str, filepath: String) {
    let mut response = reqwest::blocking::get(url).expect(format!("Couldn't recieve JSON manifest {}, please check your internet connection or the manifest URL.", url).as_str());
    let mut file = std::fs::File::create(filepath).expect("Failed creating filepath");

    let _ = std::io::copy(&mut response, &mut file).expect("silly");
}

// Return a list of versions
pub fn get_version_list(manifest: Option<&str>, ver_type: VersionType) -> Vec<String> {
    let url: &str;
    match manifest {
        Some(t) => url = t,
        None => url = DEFAULT_MANIFEST,
    }

    let versions = &get_json_manifest(url)["versions"];
    let versions = versions.as_array().unwrap();

    let mut version_list = Vec::new();

    for version in versions {
        let id: String = version["id"].to_string().replace('"', "");
        let release_type: String = version["type"].to_string().replace('"', "");
        match ver_type {
            VersionType::Release => match release_type.as_str() {
                "release" => version_list.push(id),
                _ => (),
            },
            VersionType::Snapshot => match release_type.as_str() {
                "snapshot" => version_list.push(id),
                _ => (),
            },
        }
    }

    return version_list;
}

pub fn save_server_file_from_index(item: usize, filepath: &str, ver_type: VersionType) {
    let i: usize = (item).try_into().expect("Failed parsing version index");
    let list = get_version_list(None, ver_type);
    download_server_jar(Some(list[i].as_str()), filepath, None);
}

pub fn return_ver_from_index(item: usize, ver_type: VersionType) -> String {
    let i: usize = (item).try_into().unwrap();
    let list = get_version_list(None, ver_type);
    return list[i].to_string();
}

//pub fn return_index_from_ver(ver: String) -> usize {
//    let (list, _) = get_version_list(None);
//    list.iter().position(|n| *n == ver).unwrap()
//}
