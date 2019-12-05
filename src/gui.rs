use super::collection;

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

pub fn render(collection: collection::Collection) {
    let application =
        gtk::Application::new(Some("it.me"), Default::default()).expect("GTK app init failed...");

    application.connect_activate(move |app| build_ui(app, collection.clone()));

    application.run(&[]);
}

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
    add_ops(&main_vbox);
    add_quiz_button(&main_vbox);

    window.add(&main_vbox);
    window.show_all();
}

fn add_decks(vbox: &gtk::Box, collection: &collection::Collection) {
    let scroll_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let deck_box = gtk::Box::new(gtk::Orientation::Vertical, collection.decks.len() as i32);

    for deck in &collection.decks {
        let deck_button = gtk::ButtonBuilder::new().label(&deck.title.clone()).build();
        deck_box.add(&deck_button);
    }

    scroll_window.add(&deck_box);
    vbox.add(&scroll_window);
}

fn add_ops(vbox: &gtk::Box) {
    let ops = gtk::Grid::new();
    ops.set_row_homogeneous(true);
    ops.set_column_homogeneous(true);

    vbox.add(&ops);
}

fn add_quiz_button(vbox: &gtk::Box) {
    let quiz_button = gtk::Button::new_with_label("Quiz Me!");
    quiz_button.connect_clicked(|_btn| {
        println!("Clicked quiz button");
    });
    vbox.add(&quiz_button);
}
