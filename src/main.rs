mod manage_server;
mod save_server;
use cursive::{Cursive, CursiveExt, view::Resizable, views::*};

use crate::manage_server::*;
use crate::save_server::*;

fn main() {
    let mut siv = Cursive::default();

    let mut select_view = cursive::views::SelectView::new();
    let (release_list, _snapshot_list) = get_version_list(None);

    let mut index = 0;
    for ver in release_list {
        select_view.add_item(ver, index);
        index += 1;
    }

    select_view.set_on_submit(|s: &mut Cursive, i: &i32| {
        let i = i.clone();
        let edit = Dialog::new().title("Enter a server name").content(
            EditView::new()
                .content(format!("Vanilla{}", return_ver_from_index(i)))
                .on_submit(move |s, text| {
                    create_server_directory(text);
                    save_server_file_from_index(
                        i,
                        format!(
                            r#"{home}{sep}servers{sep}{server_name}{sep}server.jar"#,
                            sep = std::path::MAIN_SEPARATOR_STR,
                            home = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                            server_name = text
                        )
                        .as_str(),
                    );
                    agree_to_eula(text);

                    s.pop_layer();
                })
                .fixed_width(25),
        );
        s.add_layer(edit);
    });

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_width(scroll_view);
    let title_view = Panel::new(resized_view).title("Select a sever version to download (Vanilla)");

    siv.add_layer(title_view);
    siv.run();
}

pub fn return_ver_from_index(item: i32) -> String {
    let i: usize = (item).try_into().unwrap();
    let (list, _) = get_version_list(None);
    return list[i].to_string();
}
