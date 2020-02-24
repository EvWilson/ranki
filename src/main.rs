use std::error;
use std::sync::mpsc;
use std::thread;

mod collection;
mod gui;

fn main() -> Result<(), Box<dyn error::Error>> {
    let (tx, rx) = mpsc::channel();

    let mut collection_service = collection::CollectionService::new()?;

    thread::spawn(move || {
        collection_service.listen(rx);
    });

    gui::render(tx);

    Ok(())
}
