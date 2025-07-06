use crate::create_server::*;
use crate::manage_server::*;
use cursive::{Cursive, view::Resizable, views::*};

fn download_server(siv: &mut Cursive, ver_type: VersionType) {
    let version_list = get_version_list(None, ver_type);

    let mut select_view: SelectView<usize> = cursive::views::SelectView::new();

    for (index, ver) in version_list.iter().enumerate() {
        select_view.add_item(ver.clone(), index);
    }

    // Ugly code but easiest way to pass version type to contructor
    match ver_type {
        VersionType::Release => select_view.set_on_submit(|s: &mut Cursive, i: &usize| {
            let i = i.clone();
            let edit = Dialog::new().title("Enter a server/folder name").content(
                EditView::new()
                    .content(format!("Vanilla{}", return_ver_from_index(i, VersionType::Release)))
                    .on_submit(move |s, text| {
                        // Create a new server for the downloaded server to be saved
                        let mut server = ServerInstance::new(
                            text.to_string(),
                            String::new(),
                            return_ver_from_index(i, VersionType::Release),
                            VersionType::Release,
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
                            VersionType::Release
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
        }),
        VersionType::Snapshot => select_view.set_on_submit(|s: &mut Cursive, i: &usize| {
            let i = i.clone();
            let edit = Dialog::new().title("Enter a server/folder name").content(
                EditView::new()
                    .content(format!("Vanilla{}", return_ver_from_index(i, VersionType::Snapshot)))
                    .on_submit(move |s, text| {
                        // Create a new server for the downloaded server to be saved
                        let mut server = ServerInstance::new(
                            text.to_string(),
                            String::new(),
                            return_ver_from_index(i, VersionType::Snapshot),
                            VersionType::Snapshot,
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
                            VersionType::Snapshot
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
        }),
    }

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_width(scroll_view);
    let title_view =
        Panel::new(resized_view).title("Select a server version to download (Vanilla)");

    siv.add_layer(title_view);
}

pub fn select_version_type(siv: &mut Cursive) {
    let mut select = SelectView::new();
    select.add_item("Release", 0);
    select.add_item("Snapshot", 1);
    select.set_on_submit(|s, value| {
        match value {
            0 => download_server( s , VersionType::Release),
            1 => download_server( s , VersionType::Snapshot),
            _ => ()
        }
    });

    let title = Panel::new(select).title("Select the type of game version you want to download");
    siv.add_layer(title);
}
