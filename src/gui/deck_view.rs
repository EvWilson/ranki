use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::*;

use relm::{Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::deck_operations::DeckOpGrid;
use super::window::Msg;
use crate::collection::Collection;

// ----- Deck View Widget -----
// A scrolled window containing all the decks currently in the user's collection
pub struct DeckViewModel {
    active_deck: Option<u32>,
    collection: Rc<RefCell<Collection>>,
    deck_map: HashMap<u32, Component<DeckWidget>>,
    parent_stream: EventStream<Msg>,
    stream: EventStream<DeckViewMsg>,
}

#[derive(Msg)]
pub enum DeckViewMsg {
    Add(String),
    Delete,
    Edit(String),
    NewCollection,
    Selected(u32),
    Cleared,
}

#[widget]
impl Widget for DeckView {
    fn model(
        relm: &Relm<Self>,
        (collection, parent_stream): (Rc<RefCell<Collection>>, EventStream<Msg>),
    ) -> DeckViewModel {
        DeckViewModel {
            active_deck: None,
            collection,
            deck_map: HashMap::new(),
            parent_stream,
            stream: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: DeckViewMsg) {
        match event {
            DeckViewMsg::Add(name) => {
                self.model.parent_stream.emit(Msg::AddDeck(name));
            }
            DeckViewMsg::Delete => {
                if let Some(id) = self.model.active_deck {
                    self.model.parent_stream.emit(Msg::DeleteDeck(id));
                }
            }
            DeckViewMsg::Edit(name) => {
                self.model.parent_stream.emit(Msg::EditDeck(name));
            }
            DeckViewMsg::NewCollection => {
                self.update_model();
            }
            DeckViewMsg::Selected(id) => {
                self.model.active_deck = Some(id);
                self.active_deck_label
                    .set_text(&format!("Selected deck: {}", id));
                self.model.parent_stream.emit(Msg::SelectedDeck(Some(id)));
            }
            DeckViewMsg::Cleared => {
                self.model.active_deck = None;
                self.active_deck_label.set_text("Selected deck: None");
                self.model.parent_stream.emit(Msg::SelectedDeck(None));
            }
        }
    }

    fn update_model(&mut self) {
        for (_id, widget) in self.model.deck_map.drain() {
            self.decks.remove_widget(widget);
        }
        if let Ok(c) = self.model.collection.try_borrow() {
            for deck in &c.decks {
                let widget = self.decks.add_widget::<DeckWidget>((
                    deck.id,
                    deck.title.clone(),
                    self.model.stream.clone(),
                ));
                self.model.deck_map.insert(deck.id, widget);
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Decks",
            },
            #[name="active_deck_label"]
            gtk::Label {},
            gtk::ScrolledWindow {
                min_content_height: 80,
                shadow_type: gtk::ShadowType::Out,
                #[name="decks"]
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                },
            },
            #[name="deck_op_grid"]
            DeckOpGrid(self.model.stream.clone()),
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
        let mut name = deck_name;
        if name.chars().count() > 20 {
            name = name.chars().take(20).collect::<String>();
            name.push_str("...");
        }
        DeckModel {
            id,
            deck_name: format!("{}: {}", id, name),
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
            },
        }
    }
}
