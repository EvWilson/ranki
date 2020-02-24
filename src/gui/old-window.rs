use std::sync::mpsc;

use gio::prelude::*;
use gtk::prelude::*;

use crate::collection::store;

#[derive(Clone)]
pub struct GuiMgr {
    tx: mpsc::Sender<store::Action>,
}

impl GuiMgr {
    pub fn new(tx: mpsc::Sender<store::Action>) -> Self {
        GuiMgr { tx }
    }
}

enum Operations {
    AddDeck,
    DeleteDeck,
}

pub fn render(mgr: GuiMgr) {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.clock"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(move |app| {
        build_ui(app, &mgr.tx);
    });

    application.run(&vec![]);
}

fn build_ui(application: &gtk::Application, _tx: &mpsc::Sender<store::Action>) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("anki-rs");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let main_vbox = gtk::Box::new(gtk::Orientation::Vertical, 3);

    add_decks(&main_vbox);
    add_op_buttons(&main_vbox);

    window.add(&main_vbox);
    window.show_all();
}

// Add all the decks to a scrollable window
fn add_decks(vbox: &gtk::Box) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let decks_label = gtk::Label::new(Some("Decks"));
    container.add(&decks_label);

    let scroll_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    /*
    let deck_box = gtk::Box::new(gtk::Orientation::Vertical, collection.decks.len() as i32);

    for deck in &collection.decks {
        let deck_button = gtk::ButtonBuilder::new().label(&deck.title.clone()).build();
        let deck_title = deck.title.clone();
        deck_button.connect_clicked(move |_btn| {
            println!("Clicked deck button with title: {}", deck_title);
        });
        deck_box.add(&deck_button);
    }

    scroll_window.add(&deck_box);
    */
    container.add(&scroll_window);
    vbox.add(&container);
}

// Buttons relating to what operations you can perform on the buttons
fn add_op_buttons(vbox: &gtk::Box) {
    let op_box = gtk::Box::new(gtk::Orientation::Vertical, 2);

    let op_label = gtk::Label::new(Some("Deck Operations"));
    op_box.add(&op_label);

    let ops = gtk::Grid::new();
    ops.set_row_homogeneous(true);
    ops.set_column_homogeneous(true);

    let add_button = gtk::Button::new_with_label("Add");
    let vbox_clone = vbox.clone();
    add_button.connect_clicked(move |_btn| {
        println!("Clicked add deck button");
        /*
        let send_res = &tx.send(store::Action::AddDeck("hello".to_string()));
        match send_res {
            Ok(_) => {
                println!("Msg sent successfully");
            }
            Err(e) => {
                println!("Error in send: {}", e);
            }
        }
        */
        let modal = operations_modal(Operations::AddDeck);
        vbox_clone.add(&modal);
        vbox_clone.show_all();
    });
    ops.attach(&add_button, 0, 0, 1, 1);

    let delete_button = gtk::Button::new_with_label("Delete");
    let vbox_clone = vbox.clone();
    delete_button.connect_clicked(move |_btn| {
        println!("Clicked delete deck button");
        let modal = operations_modal(Operations::DeleteDeck);
        vbox_clone.add(&modal);
        vbox_clone.show_all();
    });
    ops.attach(&delete_button, 1, 0, 1, 1);

    let edit_button = gtk::Button::new_with_label("Edit");
    edit_button.connect_clicked(|_btn| {
        println!("Clicked edit deck button");
    });
    ops.attach(&edit_button, 0, 1, 1, 1);

    op_box.add(&ops);
    vbox.add(&op_box);
}

// Modal for performing operations when one is selected for a given deck
fn operations_modal(op: Operations) -> gtk::Box {
    let op_box = gtk::Box::new(gtk::Orientation::Vertical, 1);

    match op {
        Operations::AddDeck => {
            println!("add deck operation");
            let op_label = gtk::Label::new(Some("Add Deck"));
            op_box.add(&op_label);
            let op_label = gtk::Label::new(Some("Name of Deck:"));
            op_box.add(&op_label);
        }
        Operations::DeleteDeck => {
            println!("delete deck operation");
            let op_label = gtk::Label::new(Some("Delete Deck"));
            op_box.add(&op_label);
        }
    }

    op_box
}
