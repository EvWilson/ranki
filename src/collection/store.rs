use std::error::Error;
use std::sync::mpsc;

use super::data;

pub enum Action {
    AddDeck(String),
}

pub struct CollectionService {
    //collection: Arc<Mutex<data::Collection>>,
    collection: data::Collection,
    tx: mpsc::Sender<data::Collection>,
}

impl CollectionService {
    pub fn new(tx: mpsc::Sender<data::Collection>) -> Result<CollectionService, Box<dyn Error>> {
        let collection = data::Collection::load_from_file()?;

        Ok(CollectionService {
            //collection: Arc::new(Mutex::new(collection)),
            collection,
            tx,
        })
    }

    pub fn listen(&mut self, rx: mpsc::Receiver<Action>) {
        for event in rx {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, action: Action) {
        match action {
            Action::AddDeck(deck_name) => {
                println!("Got a request to add a deck of name {}", deck_name);
                self.collection.add_deck(&deck_name);
                if let Err(e) = self.send_update() {
                    println!("error sending collection update: {}", e);
                }
            }
        }
    }

    fn send_update(&self) -> Result<(), mpsc::SendError<data::Collection>> {
        self.tx.send(self.collection.clone())
    }
}
