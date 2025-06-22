mod download;
use cursive::{view::{Resizable}, views::*, Cursive, CursiveExt};

use crate::download::{download_server_jar, get_version_list};

fn main() {
    let mut siv = Cursive::default();

    
    let mut select_view = cursive::views::SelectView::new();
    let (release_list ,_snapshot_list) = get_version_list(None);

    let mut index = 0;
    for ver in release_list {
        select_view.add_item(ver, index);
        index += 1;
    }
    
    select_view.set_on_submit(|s: &mut Cursive, i: &i32|{
        let i = i.clone();
        let edit = Dialog::new()
            .content(EditView::new()
                .on_submit(move |s, text| {
                    save_selected_file(i.clone(), text);
                    s.pop_layer();})
                .fixed_width(25)
            );
        s.add_layer(edit);
    });

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_width(scroll_view);

    siv.add_layer(resized_view);
    siv.run();
}

fn save_selected_file(item: i32, filepath: &str) {
    let i: usize =  (item).try_into().unwrap();
    let (snapshot_list, _) = get_version_list(None);
    download_server_jar(Some(snapshot_list[i].as_str()), filepath, None);
}
