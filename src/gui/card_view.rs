use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::*;

use relm::{Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::card_operations::CardOpGrid;
use super::window;
use crate::collection::Collection;

pub struct CardViewModel {
    active_card: Option<u32>,
    card_map: HashMap<u32, Component<CardWidget>>,
    collection: Rc<RefCell<Collection>>,
    parent_stream: EventStream<window::Msg>,
    stream: EventStream<CardViewMsg>,
}

#[derive(Msg)]
pub enum CardViewMsg {
    Add(String, String),
    Delete,
    Edit(String, String),
    Selected(u32),
    Cleared,
    UpdateToDeck(u32),
}

#[widget]
impl Widget for CardView {
    fn model(
        relm: &Relm<Self>,
        (collection, parent_stream): (Rc<RefCell<Collection>>, EventStream<window::Msg>),
    ) -> CardViewModel {
        CardViewModel {
            active_card: None,
            card_map: HashMap::new(),
            collection,
            parent_stream,
            stream: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: CardViewMsg) {
        match event {
            CardViewMsg::Add(question, answer) => {
                self.model
                    .parent_stream
                    .emit(window::Msg::AddCard(question, answer));
            }
            CardViewMsg::Delete => {
                if let Some(id) = self.model.active_card {
                    self.model.parent_stream.emit(window::Msg::DeleteCard(id));
                }
            }
            CardViewMsg::Edit(question, answer) => {
                if let Some(id) = self.model.active_card {
                    self.model
                        .parent_stream
                        .emit(window::Msg::EditCard(id, question, answer));
                }
            }
            CardViewMsg::Selected(id) => {
                self.model.active_card = Some(id);
                self.active_card_label
                    .set_text(&format!("Selected card: {}", id));
            }
            CardViewMsg::Cleared => {
                self.model.active_card = None;
                self.active_card_label.set_text("Selected card: None");
            }
            CardViewMsg::UpdateToDeck(id) => {
                for (_id, widget) in self.model.card_map.drain() {
                    self.cards.remove_widget(widget);
                }
                if let Ok(c) = self.model.collection.try_borrow() {
                    let deck_pos = c.deck_pos_by_id(id);
                    let deck = match deck_pos {
                        Some(pos) => &c.decks[pos],
                        None => {
                            println!("error: could not find deck of id {}", id);
                            return;
                        }
                    };
                    for card in &deck.cards {
                        let widget = self.cards.add_widget::<CardWidget>((
                            card.question.clone(),
                            card.id,
                            self.model.stream.clone(),
                        ));
                        self.model.card_map.insert(card.id, widget);
                    }
                }
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                label: "Cards",
            },
            #[name="active_card_label"]
            gtk::Label {},
            gtk::ScrolledWindow {
                min_content_height: 80,
                shadow_type: gtk::ShadowType::Out,
                #[name="cards"]
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                },
            },
            #[name="card_op_grid"]
            CardOpGrid(self.model.stream.clone()),
        }
    }
}

pub struct CardModel {
    card_name: String,
    id: u32,
    parent_stream: EventStream<CardViewMsg>,
}

#[derive(Msg)]
pub enum CardMsg {
    Selected,
}

#[widget]
impl Widget for CardWidget {
    fn model((card_name, id, parent_stream): (String, u32, EventStream<CardViewMsg>)) -> CardModel {
        let mut name = card_name;
        if name.chars().count() > 20 {
            name = name.chars().take(20).collect::<String>();
            name.push_str("...");
        }
        CardModel {
            card_name: format!("{}: {}", id, name),
            id,
            parent_stream,
        }
    }

    fn update(&mut self, event: CardMsg) {
        match event {
            CardMsg::Selected => {
                self.model
                    .parent_stream
                    .emit(CardViewMsg::Selected(self.model.id));
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Button {
                label: &self.model.card_name,
                clicked => CardMsg::Selected,
            },
        }
    }
}
