use super::collection;

extern crate gio;
extern crate gtk;
use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

struct UISettings {
    width: i32,
    height: i32,
}

const SETTINGS: UISettings = UISettings {
    width: 640,
    height: 480,
};

pub fn render(collection: collection::Collection) {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(SETTINGS.width, SETTINGS.height);

        for deck in &collection.decks {
            let button = Button::new_with_label(&deck.title);
            button.connect_clicked(|_| {
                println!("Clicked deck with name {}", "blah");
                deck.card_by_index(1);
            });
            window.add(&button);
        }

        window.show_all();
    });

    application.run(&[]);
}
