mod create_server;
mod manage_server;
mod menus;

use cursive::{Cursive, CursiveExt, views::*};
use crate::{manage_server::VersionType, menus::*};

fn main() {
    let mut siv = Cursive::default();

    let options: [&str; 4] = ["Download new server", "Run server", "Edit server properties", "Program settings"];

    let mut select = SelectView::new();
    for (index, item) in options.iter().enumerate() {
        select.add_item(item.to_string(), index);
    }
    select.set_on_submit(|s: &mut Cursive, i: &usize| {
        match i {
            0 => download_server( s , VersionType::Release),
            _ => ()
        }
    });

    siv.add_layer(select);

    siv.run();
}
