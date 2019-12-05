use super::collection;

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

pub fn render(_collection: collection::Collection) {
    let application = gtk::Application::new(Some("it.me"), Default::default())
        .expect("GTK app init failed...");

    application.connect_activate(|app| {build_ui(app)});

    application.run(&[]);
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("First GTK+ Program")
        .border_width(10)
        .position(gtk::WindowPosition::Center)
        .default_size(350, 70);

    let button = gtk::Button::new_with_label("Click me!");

    window.add(&button);

    window.show_all();
}
