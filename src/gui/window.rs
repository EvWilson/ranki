use std::rc::Rc;
use std::sync::mpsc;

use gtk::*;
use relm::{EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::deck_view::DeckView;
use super::operations_grid::OpGrid;
use crate::collection;

pub type CollectionSender = mpsc::Sender<collection::Action>;
pub type UpdateReceiver = mpsc::Receiver<collection::Collection>;

pub fn render(tx: CollectionSender, rx: UpdateReceiver) {
    Win::run((tx, rx)).expect("Win::run failed");
}

pub struct Model {
    tx: CollectionSender,
    selected_deck: Option<u32>,
    stream: EventStream<Msg>,
    rx: Rc<UpdateReceiver>,
}

#[derive(Msg)]
pub enum Msg {
    SelectedDeck(u32),
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, (tx, rx): (CollectionSender, UpdateReceiver)) -> Model {
        Model {
            tx,
            selected_deck: None,
            stream: relm.stream().clone(),
            rx: Rc::new(rx),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SelectedDeck(id) => {
                self.model.selected_deck = Some(id);
            }
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
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                #[name="deck_view"]
                DeckView(self.model.stream.clone(), self.model.rx.clone()),
                #[name="op_grid"]
                OpGrid(self.model.tx.clone()),
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
