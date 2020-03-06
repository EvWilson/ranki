use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use gtk::*;
use relm::{interval, Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use super::card_view::{CardView, CardViewMsg};
use super::deck_view::{DeckView, DeckViewMsg};
use super::quiz::{QuizMsg, QuizView};
use crate::collection;

pub type CollectionSender = mpsc::Sender<collection::Action>;
pub type UpdateReceiver = mpsc::Receiver<collection::SendData>;

pub fn render(tx: CollectionSender, rx: UpdateReceiver) {
    Win::run((tx, rx)).expect("Win::run failed");
}

pub struct Model {
    card_view: Option<Component<CardView>>,
    collection: Rc<RefCell<collection::Collection>>,
    tx: CollectionSender,
    selected_deck: Option<u32>,
    stream: EventStream<Msg>,
    rx: Rc<UpdateReceiver>,
}

#[derive(Msg)]
pub enum Msg {
    AddDeck(String),
    DeleteDeck(u32),
    EditDeck(String),
    AddCard(String, String),
    DeleteCard(u32),
    EditCard(u32, String, String),
    SelectedDeck(Option<u32>),
    StartQuiz,
    Tick,
    QuizComplete(Vec<(u32, u32, Option<bool>)>),
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, (tx, rx): (CollectionSender, UpdateReceiver)) -> Model {
        Model {
            card_view: None,
            collection: Rc::new(RefCell::new(collection::Collection::new())),
            tx,
            selected_deck: None,
            stream: relm.stream().clone(),
            rx: Rc::new(rx),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddDeck(name) => {
                if let Err(e) = self.model.tx.send(collection::Action::AddDeck(name)) {
                    println!("error sending add deck msg to collection mgr: {}", e);
                }
            }
            Msg::DeleteDeck(id) => {
                if let Err(e) = self.model.tx.send(collection::Action::DeleteDeck(id)) {
                    println!(
                        "error: while sending delete deck message to collection mgr: {}",
                        e
                    );
                }
            }
            Msg::EditDeck(name) => match self.model.selected_deck {
                Some(id) => {
                    if let Err(e) = self
                        .model
                        .tx
                        .send(collection::Action::EditDeck(id, name.clone()))
                    {
                        println!(
                            "error sending edit deck msg to collection mgr. id {}, new name {}, error {}",
                            id, name, e
                        );
                    }
                }
                None => {
                    println!(
                        "error: received edit deck msg when no deck active. deck name: {}",
                        name
                    );
                }
            },
            Msg::AddCard(question, answer) => match self.model.selected_deck {
                Some(deck_id) => {
                    if let Err(e) = self.model.tx.send(collection::Action::AddCard(
                        deck_id,
                        question.clone(),
                        answer.clone(),
                    )) {
                        println!(
                            "error sending add card msg to collection mgr. deck_id {} q {} a {} error {}",
                            deck_id, question, answer, e
                        );
                    }
                }
                None => {
                    println!("error: received add card msg when no deck active");
                }
            },
            Msg::DeleteCard(card_id) => match self.model.selected_deck {
                Some(deck_id) => {
                    if let Err(e) = self
                        .model
                        .tx
                        .send(collection::Action::DeleteCard(deck_id, card_id))
                    {
                        println!(
                            "error sending delete card msg to collection mgr. deck_id {} card_id {} error {}",
                            deck_id, card_id, e
                        );
                    }
                }
                None => {
                    println!("error: received delete card msg when no deck active");
                }
            },
            Msg::EditCard(card_id, question, answer) => match self.model.selected_deck {
                Some(deck_id) => {
                    if let Err(e) = self.model.tx.send(collection::Action::EditCard(
                        deck_id,
                        card_id,
                        question.clone(),
                        answer.clone(),
                    )) {
                        println!(
                            "error sending edit card msg to collection mgr. deck_id {} card_id {} q{} a{} error {}",
                            deck_id, card_id, question, answer, e
                        );
                    }
                }
                None => {
                    println!("error: received edit card msg when no deck active");
                }
            },
            Msg::StartQuiz => {
                if let Err(e) = self.model.tx.send(collection::Action::GetQuiz) {
                    println!("error sending get quiz msg: {}", e);
                }
            }
            Msg::SelectedDeck(id) => {
                if let Some(widget) = self.model.card_view.take() {
                    self.card_view_box.remove_widget(widget);
                }
                if let Some(id) = id {
                    self.model.selected_deck = Some(id);
                    let widget = self.card_view_box.add_widget::<CardView>((
                        self.model.collection.clone(),
                        self.model.stream.clone(),
                    ));
                    widget.emit(CardViewMsg::UpdateToDeck(id));
                    self.model.card_view = Some(widget);
                } else {
                    self.model.selected_deck = None;
                }
            }
            Msg::Tick => match self.model.rx.try_recv() {
                Err(mpsc::TryRecvError::Empty) => {}
                Ok(collection::SendData::Collection(collection)) => {
                    self.model.collection.replace(collection);
                    self.deck_view.emit(DeckViewMsg::NewCollection);
                    if let Some(id) = self.model.selected_deck {
                        if let Some(card_view) = &self.model.card_view {
                            card_view.emit(CardViewMsg::UpdateToDeck(id));
                        }
                    }
                }
                Ok(collection::SendData::Quiz(quiz)) => {
                    self.quiz_view.emit(QuizMsg::GotQuiz(quiz));
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("Window receiver disconnected");
                }
            },
            Msg::QuizComplete(results) => {
                if let Err(e) = self.model.tx.send(collection::Action::QuizResults(results)) {
                    println!("error sending quiz results to collection mgr. error: {}", e);
                }
            }
            Msg::Quit => {
                gtk::main_quit();
            }
        }
        self.shrink_to_fit();
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        interval(relm.stream(), 1000, || Msg::Tick);
    }

    fn shrink_to_fit(&self) {
        self.window.resize(1, 1);
    }

    view! {
        #[name="window"]
        gtk::Window {
            title: "anki-rs",
            border_width: 10,
            position: gtk::WindowPosition::Center,
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                #[name="deck_view"]
                DeckView(self.model.collection.clone(), self.model.stream.clone()),
                #[name="card_view_box"]
                gtk::Box {},
                #[name="quiz_view"]
                QuizView(self.model.stream.clone()),
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
