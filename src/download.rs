use reqwest::blocking::Response;
use std::fs::File;
use std::io;

pub fn get_json_manifest(url: &str) -> serde_json::Value {
    let manifest:Response = reqwest::blocking::get(url).unwrap();

    let manifest_json:serde_json::Value = serde_json::from_str(&manifest.text().unwrap()).unwrap();
    return manifest_json;
}

pub fn get_latest_version() -> String {
    let manifest: serde_json::Value = get_json_manifest("https://launchermeta.mojang.com/mc/game/version_manifest.json");
    return manifest["latest"]["release"].to_string().replace('"',"");
}

pub fn get_version_url(version: &str) -> Option<String> {
    let manifest: serde_json::Value = get_json_manifest("https://launchermeta.mojang.com/mc/game/version_manifest.json");
    let versions: Option<&Vec<serde_json::Value>> = manifest["versions"].as_array();

    for id in versions.unwrap() {
        let ver: String = id["id"].to_string().replace('"',"");

        if version == ver {
            
            let url: String = id["url"].to_string().replace('"', "");
            println!("found {}", url);
            return Some(url);
        }
    }

    return None;
}

pub fn save_file(url: &str, filepath: &str) {
    let mut response = reqwest::blocking::get(url).unwrap();
    let mut file = File::create(filepath).unwrap();
    io::copy(&mut response, &mut file).unwrap();
    print!("Saved file as {}", filepath);
} 