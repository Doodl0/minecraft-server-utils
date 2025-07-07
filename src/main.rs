mod create_server;
mod menus;
mod server_instance;

use crate::menus::{select_saved_server, select_version_type};
use cursive::{Cursive, CursiveExt, views::*};

const MENU_OPTIONS: [&str; 4] = [
    "Download new server",
    "Manage existing server",
    "Program settings",
    "Exit",
];

fn main() {
    let mut siv = Cursive::default();

    let mut select = SelectView::new();
    for (index, item) in MENU_OPTIONS.iter().enumerate() {
        select.add_item(item.to_string(), index);
    }
    select.set_on_submit(|s: &mut Cursive, i: &usize| match i {
        0 => select_version_type(s),
        1 => select_saved_server(s),
        3 => s.quit(),
        _ => (),
    });

    siv.add_layer(select);
    siv.run();
}
