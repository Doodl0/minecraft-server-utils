mod download;
use crate::download::*;

fn main() {
    let version_url = get_version_url(&get_latest_version()).unwrap().replace('"', "");

    let download_link =
        get_json_manifest(&version_url.as_str())["downloads"]["server"]["url"].to_string().replace('"', "");
    println!("Found {}", download_link);

    save_file(&download_link, r#"server.jar"#);
}
