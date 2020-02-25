use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc;

use gtk::*;

use relm::{interval, Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::window::{Msg, UpdateReceiver};
use crate::collection::Collection;

// ----- Deck View Widget -----
// A scrolled window containing all the decks currently in the user's collection
pub struct DeckViewModel {
    active_deck_id: Option<u32>,
    collection: Option<Collection>,
    deck_map: HashMap<u32, Component<DeckWidget>>,
    parent_stream: EventStream<Msg>,
    stream: EventStream<DeckViewMsg>,
    rx: Rc<UpdateReceiver>,
}

#[derive(Msg)]
pub enum DeckViewMsg {
    Tick,
    Selected(u32),
}

#[widget]
impl Widget for DeckView {
    fn model(
        relm: &Relm<Self>,
        (parent_stream, rx): (EventStream<Msg>, Rc<UpdateReceiver>),
    ) -> DeckViewModel {
        DeckViewModel {
            active_deck_id: None,
            collection: None,
            deck_map: HashMap::new(),
            parent_stream,
            stream: relm.stream().clone(),
            rx,
        }
    }

    fn update(&mut self, event: DeckViewMsg) {
        match event {
            DeckViewMsg::Tick => {
                loop {
                    match self.model.rx.try_recv() {
                        Err(mpsc::TryRecvError::Empty) => {
                            break;
                        }
                        Ok(collection) => {
                            self.update_model(collection);
                        }
                        Err(mpsc::TryRecvError::Disconnected) => {
                            panic!("error: deck view receiver has been disconnected, panicking");
                            //break;
                        }
                    }
                }
            }
            DeckViewMsg::Selected(id) => {
                self.model.active_deck_id = Some(id);
                self.active_deck_label
                    .set_text(&format!("Selected deck: {}", id));
                self.model.parent_stream.emit(Msg::SelectedDeck(id));
            }
        }
    }

    fn update_model(&mut self, collection: Collection) {
        if let Some(c) = &mut self.model.collection {
            // Add deck if we don't have it already
            for deck in &collection.decks {
                if !c.contains_deck_id(deck.id) {
                    c.decks.push(deck.clone());
                    let widget = self.decks.add_widget::<DeckWidget>((
                        deck.id,
                        deck.title.clone(),
                        self.model.stream.clone(),
                    ));
                    self.model.deck_map.insert(deck.id, widget);
                }
            }
            // Remove deck if it isn't in the update collection
            let mut remove_ids = vec![];
            for deck in &mut c.decks {
                if !collection.contains_deck_id(deck.id) {
                    if let Some(widget) = self.model.deck_map.remove(&deck.id) {
                        self.decks.remove_widget(widget);
                        remove_ids.push(deck.id);
                    } else {
                        println!("error: couldn't find deck widget in deck view widget map");
                    }
                }
            }
            for id in remove_ids {
                if let None = c.remove_deck_by_id(id) {
                    println!("error: could not find deck to remove by id: {}", id);
                }
            }
        } else {
            // If we didn't have a collection before, update is easy
            // Just take entire collection as new state
            for deck in &collection.decks {
                let widget = self.decks.add_widget::<DeckWidget>((
                    deck.id,
                    deck.title.clone(),
                    self.model.stream.clone(),
                ));
                self.model.deck_map.insert(deck.id, widget);
            }
            self.model.collection = Some(collection);
            return;
        }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        interval(relm.stream(), 1000, || DeckViewMsg::Tick);
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Decks",
            },
            #[name="active_deck_label"]
            gtk::Label {},
            #[name="decks"]
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
            }
        }
    }
}

// ----- Individual Deck Widget -----
// This is the widget to display an individual deck
pub struct DeckModel {
    id: u32,
    deck_name: String,
    parent_stream: EventStream<DeckViewMsg>,
}

#[derive(Msg)]
pub enum DeckMsg {
    Selected,
}

#[widget]
impl Widget for DeckWidget {
    fn model((id, deck_name, parent_stream): (u32, String, EventStream<DeckViewMsg>)) -> DeckModel {
        DeckModel {
            id,
            deck_name,
            parent_stream,
        }
    }

    fn update(&mut self, event: DeckMsg) {
        match event {
            DeckMsg::Selected => {
                self.model
                    .parent_stream
                    .emit(DeckViewMsg::Selected(self.model.id));
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Button {
                label: &self.model.deck_name,
                clicked => DeckMsg::Selected,
            }
        }
    }
}
