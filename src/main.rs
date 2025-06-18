mod download;
use cursive::{views::*, Cursive, CursiveExt};

use crate::download::{download_server_jar, get_version_list};

fn main() {
    let mut siv = Cursive::default();

    
    let mut select_view = cursive::views::SelectView::new();
    let (release_list, snapshot_list) = get_version_list(None);

    let mut index = 0;
    for ver in release_list.clone() {
        select_view.add_item(ver, index);
        index += 1;
    }

    select_view.set_on_submit(|_, item|{
        let (release_list, _) = get_version_list(None);
        download_server_jar(Some(&release_list[*item]), format!("{}.jar",&release_list[*item]), None);
    });

    let scroll_view = ScrollView::new(select_view);
    let resized_view = ResizedView::with_full_width(scroll_view);

    siv.add_layer(resized_view);
    siv.run();
}
