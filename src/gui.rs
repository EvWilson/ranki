use super::collection;

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

// Set up GTK application, build the UI, and run it
pub fn render(collection: collection::Collection) {
    let application =
        gtk::Application::new(Some("it.me"), Default::default()).expect("GTK app init failed...");

    application.connect_activate(move |app| build_ui(app, collection.clone()));

    application.run(&[]);
}

// The main layout routine
fn build_ui(application: &gtk::Application, collection: collection::Collection) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("anki-rs")
        .border_width(10)
        .window_position(gtk::WindowPosition::Center)
        .default_width(350)
        .default_height(70)
        .build();

    let main_vbox = gtk::Box::new(gtk::Orientation::Vertical, 3);

    add_decks(&main_vbox, &collection);
    add_op_buttons(&main_vbox);
    add_operations_modal(&main_vbox);
    add_quiz_button(&main_vbox);

    window.add(&main_vbox);
    window.show_all();
}

// Add all the decks to a scrollable window
fn add_decks(vbox: &gtk::Box, collection: &collection::Collection) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let decks_label = gtk::Label::new(Some("Decks"));
    container.add(&decks_label);

    let scroll_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
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
    add_button.connect_clicked(|_btn| {
        println!("Clicked add deck button");
    });
    ops.attach(&add_button, 0, 0, 1, 1);

    let delete_button = gtk::Button::new_with_label("Delete");
    delete_button.connect_clicked(|_btn| {
        println!("Clicked delete deck button");
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
// TODO
fn add_operations_modal(vbox: &gtk::Box) {}

fn add_quiz_button(vbox: &gtk::Box) {
    let quiz_button = gtk::Button::new_with_label("Quiz Me!");
    quiz_button.connect_clicked(|_btn| {
        println!("Clicked quiz button");
    });
    vbox.add(&quiz_button);
}
