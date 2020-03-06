use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::schedule::{schedule, SchedStage};

pub enum SendData {
    Collection(Collection),
    Quiz(Vec<(u32, u32, String, String)>),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Collection {
    pub id: u32,
    pub owner: String,
    pub decks: Vec<Deck>,
    curr_deck_id: u32,
}

impl Collection {
    const CONFIG_FILE: &'static str = "anki.conf";

    pub fn new() -> Self {
        Collection {
            id: 0,
            owner: "".to_string(),
            decks: Vec::new(),
            curr_deck_id: 0,
        }
    }

    pub fn add_deck(&mut self, title: &str) {
        let deck = Deck::new(self.curr_deck_id, title.to_string());
        self.curr_deck_id += 1;
        self.decks.push(deck);
    }

    pub fn add_card(
        &mut self,
        deck_id: u32,
        question: String,
        answer: String,
    ) -> Result<(), String> {
        if let Some(pos) = self.deck_pos_by_id(deck_id) {
            self.decks[pos].add_card(question, answer);
            return Ok(());
        }
        return Err(format!("could not find deck by id {}", deck_id).to_string());
    }

    pub fn remove_card(&mut self, deck_id: u32, card_id: u32) -> Result<(), String> {
        if let Some(pos) = self.deck_pos_by_id(deck_id) {
            self.decks[pos].remove_card_by_id(card_id);
            return Ok(());
        }
        return Err(format!("could not find deck by id {}", deck_id).to_string());
    }

    pub fn edit_card(
        &mut self,
        deck_id: u32,
        card_id: u32,
        new_q: String,
        new_a: String,
    ) -> Result<(), String> {
        if let Some(pos) = self.deck_pos_by_id(deck_id) {
            match self.decks[pos].edit_card_by_id(card_id, new_q, new_a) {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(format!("could not find deck by id {}", deck_id))
    }

    pub fn remove_deck_by_id(&mut self, id: u32) -> Option<Deck> {
        if let Some(pos) = self.decks.iter().position(|deck| deck.id == id) {
            Some(self.decks.remove(pos))
        } else {
            None
        }
    }

    pub fn deck_pos_by_id(&self, id: u32) -> Option<usize> {
        if let Some(pos) = self.decks.iter().position(|deck| deck.id == id) {
            Some(pos)
        } else {
            None
        }
    }

    pub(super) fn get_quiz(&self) -> Vec<(u32, u32, String, String)> {
        let mut quiz = vec![];
        for deck in &self.decks {
            let card_quiz = deck.get_quiz();
            for (card_id, question, answer) in card_quiz {
                quiz.push((deck.id, card_id, question, answer));
            }
        }
        quiz
    }

    pub(super) fn process_results(
        &mut self,
        results: Vec<(u32, u32, Option<bool>)>,
    ) -> Result<(), String> {
        for result in results {
            if let Some(passed) = result.2 {
                match self.deck_pos_by_id(result.0) {
                    Some(pos) => {
                        self.decks[pos].process_result(result.1, passed)?;
                    }
                    None => {
                        return Err(format!("error: could not find deck by id: {}", result.1));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn load_from_file() -> Result<Self, Box<dyn Error>> {
        let mut config_file = match File::open(Collection::CONFIG_FILE) {
            Ok(file) => file,
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        println!("No collection configuration file found. Creating new file.");
                    }
                    _ => {
                        return Err(Box::new(error));
                    }
                }
                return Ok(Collection::new());
            }
        };

        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let collection = serde_json::from_str(&buffer)?;

        Ok(collection)
    }

    pub fn flush_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(Collection::CONFIG_FILE)?;
        let bytes = serde_json::to_vec(&self)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Deck {
    pub id: u32,
    pub title: String,
    pub cards: Vec<Card>,
    curr_card_id: u32,
}

impl Deck {
    fn new(id: u32, title: String) -> Self {
        Deck {
            id,
            title,
            cards: Vec::new(),
            curr_card_id: 0,
        }
    }

    pub fn add_card(&mut self, question: String, answer: String) {
        self.cards
            .push(Card::new(self.curr_card_id, question, answer));
        self.curr_card_id += 1;
    }

    pub fn remove_card_by_id(&mut self, card_id: u32) -> Option<Card> {
        if let Some(pos) = self.card_pos_by_id(card_id) {
            Some(self.cards.remove(pos))
        } else {
            None
        }
    }

    pub fn edit_card_by_id(
        &mut self,
        card_id: u32,
        new_q: String,
        new_a: String,
    ) -> Result<(), String> {
        if let Some(pos) = self.card_pos_by_id(card_id) {
            self.cards[pos].question = new_q;
            self.cards[pos].answer = new_a;
            return Ok(());
        } else {
            Err(format!("could not find card by id {}", card_id))
        }
    }

    fn card_pos_by_id(&mut self, card_id: u32) -> Option<usize> {
        if let Some(pos) = self.cards.iter().position(|card| card.id == card_id) {
            Some(pos)
        } else {
            None
        }
    }

    fn get_quiz(&self) -> Vec<(u32, String, String)> {
        let mut quiz = vec![];
        for card in &self.cards {
            if card.needs_quiz() {
                quiz.push((card.id, card.question.clone(), card.answer.clone()));
            }
        }
        quiz
    }

    fn process_result(&mut self, card_id: u32, passed: bool) -> Result<(), String> {
        match self.card_pos_by_id(card_id) {
            Some(pos) => {
                self.cards[pos].process_result(passed);
            }
            None => {
                return Err(format!("could not find card by id: {}", card_id));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: u32,
    pub question: String,
    pub answer: String,
    pub note: String,
    checked: SystemTime,
    stage: SchedStage,
}

impl Card {
    fn new(id: u32, question: String, answer: String) -> Self {
        Card {
            id,
            question,
            answer,
            note: "".to_string(),
            checked: SystemTime::now(),
            stage: SchedStage::New,
        }
    }

    fn needs_quiz(&self) -> bool {
        if (self.checked + self.stage.duration()) < SystemTime::now() {
            true
        } else {
            false
        }
    }

    fn process_result(&mut self, passed: bool) {
        self.stage = schedule(&self.stage, passed);
    }
}
