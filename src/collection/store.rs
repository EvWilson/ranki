use std::error::Error;
use std::sync::mpsc;

use super::data;

pub enum Action {
    AddDeck(String),
    DeleteDeck(u32),
    EditDeck(u32, String),
    AddCard(u32, String, String),
    DeleteCard(u32, u32),
    EditCard(u32, u32, String, String),
    GetQuiz,
    QuizResults(Vec<(u32, u32, Option<bool>)>),
}

pub struct CollectionService {
    collection: data::Collection,
    tx: mpsc::Sender<data::SendData>,
}

impl CollectionService {
    pub fn new(tx: mpsc::Sender<data::SendData>) -> Result<CollectionService, Box<dyn Error>> {
        let collection = data::Collection::load_from_file()?;

        Ok(CollectionService { collection, tx })
    }

    pub fn listen(&mut self, rx: mpsc::Receiver<Action>) {
        for event in rx {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, action: Action) {
        use Action::*;

        match action {
            AddDeck(deck_name) => {
                self.collection.add_deck(&deck_name);
                if let Err(e) = self.send_update() {
                    println!("error sending post add update: {}", e);
                }
            }
            DeleteDeck(id) => {
                if let None = self.collection.remove_deck_by_id(id) {
                    println!("error: couldn't remove deck of id: {}", id);
                }
                if let Err(e) = self.send_update() {
                    println!("error sending post delete update {}", e);
                }
            }
            EditDeck(id, new_name) => match self.collection.deck_pos_by_id(id) {
                Some(pos) => {
                    self.collection.decks[pos].title = new_name;
                    if let Err(e) = self.send_update() {
                        println!("error sending post edit update: {}", e);
                    }
                }
                None => {
                    println!("error editing deck of id {}, does not exist", id);
                    return;
                }
            },
            AddCard(deck_id, question, answer) => {
                match self.collection.add_card(deck_id, question, answer) {
                    Ok(_) => {
                        if let Err(e) = self.send_update() {
                            println!("error sending post add card update: {}", e);
                        }
                    }
                    Err(e) => {
                        println!("error adding card to deck id {}: {}", deck_id, e);
                        return;
                    }
                }
            }
            DeleteCard(deck_id, card_id) => match self.collection.remove_card(deck_id, card_id) {
                Ok(_) => {
                    if let Err(e) = self.send_update() {
                        println!("error sending post delete card update: {}", e);
                    }
                }
                Err(e) => {
                    println!(
                        "error deleting card id {} from deck id {}: {}",
                        card_id, deck_id, e
                    );
                    return;
                }
            },
            EditCard(deck_id, card_id, question, answer) => {
                match self
                    .collection
                    .edit_card(deck_id, card_id, question, answer)
                {
                    Ok(_) => {
                        if let Err(e) = self.send_update() {
                            println!("error sending post edit card update: {}", e);
                        }
                    }
                    Err(e) => {
                        println!(
                            "error editing card id {} in deck id {}: {}",
                            card_id, deck_id, e
                        );
                        return;
                    }
                }
            }
            GetQuiz => {
                let quiz = self.collection.get_quiz();
                match self.tx.send(data::SendData::Quiz(quiz)) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("error sending quiz: {}", e);
                    }
                }
                return;
            }
            QuizResults(results) => {
                if let Err(e) = self.collection.process_results(results) {
                    println!("error while processing quiz result: {}", e);
                }
            }
        }
        self.save();
    }

    fn send_update(&self) -> Result<(), mpsc::SendError<data::SendData>> {
        self.tx
            .send(data::SendData::Collection(self.collection.clone()))
    }

    fn save(&self) {
        if let Err(e) = self.collection.flush_to_file() {
            println!("error saving collection to file: {}", e);
        }
    }
}
