use reqwest::blocking::Response;
use std::collections::{VecDeque};
use std::fs::File;
use std::io;

const DEFAULT_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

fn get_json_manifest(url: &str) -> serde_json::Value {
    let manifest:Response = reqwest::blocking::get(url).unwrap();

    let manifest_json:serde_json::Value = serde_json::from_str(&manifest.text().unwrap()).unwrap();
    return manifest_json;
}

fn get_latest_version(url: &str) -> String {
    let manifest: serde_json::Value = get_json_manifest(url);
    return manifest["latest"]["release"].to_string().replace('"',"");
}

fn get_version_url(version: &str, url: &str) -> Option<String> {
    let manifest: serde_json::Value = get_json_manifest(url);
    let versions: Option<&Vec<serde_json::Value>> = manifest["versions"].as_array();

    for id in versions.unwrap() {
        let ver: String = id["id"].to_string().replace('"',"");

        if version == ver {
            
            let url: String = id["url"].to_string().replace('"', "");
            return Some(url);
        }
    }

    return None;
}

fn save_file(url: &str, filepath: String) {
    let mut response = reqwest::blocking::get(url).unwrap();
    let mut file = File::create(filepath).unwrap();
    io::copy(&mut response, &mut file).unwrap();
} 

pub fn download_server_jar(version: Option<&str>, filepath: String, manifest: Option<&str>) {
    let url: &str;
    let ver: String;
    match manifest {
        Some(t) => url = t,
        None => url = DEFAULT_MANIFEST
    }
    match version {
        Some(t) => ver = t.to_string(),
        None => ver = get_latest_version(url),
    }

    let version_url = get_version_url(&ver, url).unwrap().replace('"', "");

    let download_link =
        get_json_manifest(&version_url.as_str())["downloads"]["server"]["url"].to_string().replace('"', "");

    save_file(&download_link, filepath);
}

pub fn get_version_list(manifest: Option<&str>) -> (VecDeque<String>, VecDeque<String>) {
    let url: &str;
    match manifest {
        Some(t) => url = t,
        None => url = DEFAULT_MANIFEST
    }

    let versions = &get_json_manifest(url)["versions"];
    let versions = versions.as_array().unwrap();

    let mut release_list= VecDeque::new();
    let mut snapshot_list= VecDeque::new();

    for version in versions{
        let id: String = version["id"].to_string().replace('"', "");
        let release_type: String = version["type"].to_string().replace('"', "");
        match release_type.as_str(){
            "release" => release_list.push_back(id),
            "snapshot" => snapshot_list.push_back(id),
            _ => ()
        }
    }

    return (release_list, snapshot_list);
}