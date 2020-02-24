use std::sync::mpsc;

use gtk::*;
use relm::Widget;
use relm_derive::{widget, Msg};

use super::operations_grid::OpGrid;
use crate::collection;

pub type CollectionSender = mpsc::Sender<collection::Action>;

pub fn render(tx: CollectionSender) {
    Win::run(tx).expect("Win::run failed");
}

pub struct Model {
    tx: CollectionSender,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(tx: CollectionSender) -> Model {
        Model { tx }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => {
                gtk::main_quit();
            }
        }
    }

    view! {
        gtk::Window {
            title: "anki-rs",
            border_width: 10,
            position: gtk::WindowPosition::Center,
            //default_size: 350, 70,
            #[name="op_grid"]
            OpGrid(self.model.tx.clone()),
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
