extern crate gtk;
extern crate cairo;
extern crate neovim_lib;
extern crate rmp;

mod nvim;
mod ui_model;
mod ui;

use ui::Ui;
use nvim::Nvim;

fn main() {
    let ui = Ui::new();
    ui.show();

    Nvim::start(ui).expect("Can't start nvim instance");

    gtk::main();       
}

