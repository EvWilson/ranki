use std::error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Collection {
    pub owner: String,
    pub decks: Vec<Deck>,
}

impl Collection {
    const CONFIG_FILE: &'static str = "anki.conf";

    fn new() -> Self {
        Collection {
            owner: "".to_string(),
            decks: Vec::new(),
        }
    }

    pub fn add_deck(&mut self, title: &str) {
        let deck = Deck {
            title: title.to_string(),
            cards: Vec::new(),
        };

        self.decks.push(deck);
    }

    pub fn remove_deck_by_pos(&mut self, idx: usize) -> Result<Deck, Box<dyn error::Error>> {
        if idx > self.decks.len() {
            return Err(Box::new(AnkiError::IndexOutOfBounds(idx)));
        }
        Ok(self.decks.remove(idx))
    }

    pub fn load_from_file() -> Result<Self, Box<dyn error::Error>> {
        let mut config_file = match File::open(Collection::CONFIG_FILE) {
            Ok(file) => file,
            Err(error) => {
                // TODO: only create new file on NotFound error. Otherwise propogate.
                println!("error opening conf file: {:?}", error);
                return Ok(Collection::new());
            }
        };

        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let collection = serde_json::from_str(&buffer)?;

        Ok(collection)
    }

    pub fn print(&self) {
        println!("{:?}", &self);
    }

    pub fn flush_to_file(&self) -> Result<(), Box<dyn error::Error>> {
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
    pub title: String,
    pub cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        Deck {
            title: "".to_string(),
            cards: Vec::new(),
        }
    }

    pub fn card_by_index(&self, idx: usize) -> Option<&Card> {
        self.cards.get(idx)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    pub question: String,
    pub answer: String,
    pub note: String,
}

impl Card {
    fn new() -> Self {
        Card {
            question: "".to_string(),
            answer: "".to_string(),
            note: "".to_string(),
        }
    }
}

#[derive(Debug)]
enum AnkiError {
    IndexOutOfBounds(usize),
}

impl fmt::Display for AnkiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnkiError::IndexOutOfBounds(idx) => write!(f, "index {} is out of bounds", idx),
        }
    }
}

impl error::Error for AnkiError {
    fn description(&self) -> &str {
        match *self {
            AnkiError::IndexOutOfBounds(_idx) => {
                "index is outside bounds of vector under operation"
            }
        }
    }
}
