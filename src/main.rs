mod download;
use cursive::{Cursive, CursiveExt, view::Resizable, views::*};

use crate::download::{download_server_jar, get_version_list};

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

                    let _ = std::fs::create_dir_all(format!(
                        r#"{var1}\servers\{var2}\"#,
                        var1 = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                        var2 = text
                    ));
                    
                    save_selected_file(
                        i,
                        format!(
                            r#"{var1}\servers\{var2}\server.jar"#,
                            var1 = std::env::home_dir().unwrap().as_os_str().to_str().unwrap(),
                            var2 = text
                        )
                        .as_str(),
                    );

                    s.pop_layer();
                })
                .fixed_width(25),
        );
        s.add_layer(edit);
    });

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_width(scroll_view);

    siv.add_layer(resized_view);
    siv.run();
}

fn return_ver_from_index(item: i32) -> String {
    let i: usize = (item).try_into().unwrap();
    let (list, _) = get_version_list(None);
    return list[i].to_string();
}

fn save_selected_file(item: i32, filepath: &str) {
    let i: usize = (item).try_into().expect("Failed parsing version index");
    let (list, _) = get_version_list(None);
    download_server_jar(Some(list[i].as_str()), filepath, None);
}
