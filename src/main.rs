use std::error;
use std::sync::mpsc;
use std::thread;

mod collection;
mod gui;

fn main() -> Result<(), Box<dyn error::Error>> {
    let (action_tx, action_rx) = mpsc::channel();
    let (update_tx, update_rx) = mpsc::channel();

    let mut collection_service = collection::CollectionService::new(update_tx)?;

    thread::spawn(move || {
        collection_service.listen(action_rx);
    });

    gui::render(action_tx, update_rx);

    Ok(())
}
