use std::fs::OpenOptions;

use crate::create_server::*;
use crate::server_instance::*;
use cursive::{Cursive, view::Resizable, views::*};

fn download_server(siv: &mut Cursive, ver_type: VersionType) {
    let version_list = get_version_list(None, ver_type);

    let mut select_view: SelectView<usize> = cursive::views::SelectView::new();

    for (index, ver) in version_list.iter().enumerate() {
        select_view.add_item(ver.clone(), index);
    }

    select_view.set_on_submit(move |s: &mut Cursive, i: &usize| {
        let i = i.clone();
        let edit = Dialog::new().title("Enter a server/folder name").content(
            EditView::new()
                .content(format!("Vanilla{}", return_ver_from_index(i, ver_type)))
                .on_submit(move |s, text| {
                    // Create a new server for the downloaded server to be saved
                    let mut server = ServerInstance::new(
                        text.to_string(),
                        String::new(),
                        return_ver_from_index(i, ver_type),
                        ver_type,
                    );

                    // Automatically create a server in the home directory
                    server.create_server_directory(None);
                    // Add to list of created servers
                    server.add_to_server_list();

                    // Download and save server.jar
                    save_server_file_from_index(
                        i,
                        format!(
                            r#"{home}{sep}servers{sep}{server_name}{sep}server.jar"#,
                            sep = std::path::MAIN_SEPARATOR_STR,
                            home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                            server_name = text
                        )
                        .as_str(),
                        ver_type,
                    );
                    // Create a eula file and agree to it
                    server.agree_to_eula();

                    s.pop_layer();
                    s.pop_layer();
                    s.pop_layer();
                })
                .fixed_width(25),
        );
        s.add_layer(edit);
    });

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_screen(scroll_view);
    let title_view =
        Panel::new(resized_view).title("Select a server version to download (Vanilla)");

    siv.add_layer(title_view);
}

pub fn select_version_type(siv: &mut Cursive) {
    let mut select_view = SelectView::new();
    select_view.add_item("Release", 0);
    select_view.add_item("Snapshot", 1);
    select_view.set_on_submit(|s, value| match value {
        0 => download_server(s, VersionType::Release),
        1 => download_server(s, VersionType::Snapshot),
        _ => (),
    });

    let title_view =
        Panel::new(select_view).title("Select the type of game version you want to download");
    siv.add_layer(title_view);
}

pub fn select_saved_server(siv: &mut Cursive) {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("server-list.json")
        .unwrap();
    let mut select_view = SelectView::new();
    for (index, instance) in list_saved_servers(file).iter().enumerate() {
        select_view.add_item(instance.name.clone(), index);
    }

    select_view.set_on_submit(|s, index| {
        let server_list = list_saved_servers(
            OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open("server-list.json")
                .unwrap(),
        );
        manage_server(s, server_list[*index].clone());
    });

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_screen(scroll_view);
    let title_view = Panel::new(resized_view).title("Select a server instance");

    siv.add_layer(title_view);
}

fn manage_server(siv: &mut Cursive, server: ServerInstance) {
    let server_options: [&str; 5] = [
        "Run server",
        "Modify server.properties",
        "Server Info",
        "Delete server",
        "Exit",
    ];
    let mut select_view: SelectView<usize> = SelectView::new();
    for (index, item) in server_options.iter().enumerate() {
        select_view.add_item(*item, index);
    }

    select_view.set_on_submit(move |s: &mut Cursive, index: &usize| match index {
        2 => server_info_dialog(s, server.clone()),
        4 => {
            s.pop_layer();
            s.pop_layer();
        }
        _ => (),
    });

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_screen(scroll_view);
    let title_view = Panel::new(resized_view).title("Select a server option");

    siv.add_layer(title_view);
}

fn server_info_dialog(siv: &mut Cursive, server: ServerInstance) {
    let text = TextView::new(format!(
        "Name: {}\nFolder Path: {}\nVersion: {} {}",
        server.name, server.folder, server.version_type, server.version
    ));
    let dialog = Dialog::new()
        .content(text)
        .title("Server Info")
        .button("Return", |s| {
            s.pop_layer();
        });

    siv.add_layer(dialog);
}
