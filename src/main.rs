mod create_server;
mod menus;
mod server_instance;
mod settings;

use crate::menus::{program_settings, select_saved_server, select_version_type};
use cursive::{views::*, Cursive, CursiveExt};

const MENU_OPTIONS: [&str; 4] = [
    "Download new server",
    "Manage existing server",
    "Program settings",
    "Exit",
];

fn main() {
    let mut siv = Cursive::default();
    let _ = siv.load_theme_file("theme.toml");

    let mut select = SelectView::new();
    for (index, item) in MENU_OPTIONS.iter().enumerate() {
        select.add_item(item.to_string(), index);
    }
    select.set_on_submit(|s: &mut Cursive, i: &usize| match i {
        0 => select_version_type(s),
        1 => select_saved_server(s),
        2 => program_settings(s),
        3 => s.quit(),
        _ => (),
    });

    siv.add_layer(select);
    siv.run();
}
