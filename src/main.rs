mod create_server;
mod manage_server;
mod menus;

use cursive::{Cursive, CursiveExt, views::*};
use crate::{manage_server::ServerInstance, menus::*};

const MENU_OPTIONS: [&str; 4] = ["Download new server", "Run server", "Edit server properties", "Program settings"];


fn main() {
    let mut siv = Cursive::default();


    let mut select = SelectView::new();
    for (index, item) in MENU_OPTIONS.iter().enumerate() {
        select.add_item(item.to_string(), index);
    }
    select.set_on_submit(|s: &mut Cursive, i: &usize| {
        match i {
            0 => select_version_type(s),
            _ => ()
        }
    });

    siv.add_layer(select);

    siv.run();
}
