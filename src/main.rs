use std::error;

mod collection;
mod ui;
mod gui;

fn main() -> Result<(), Box<dyn error::Error>> {
    let collection = collection::Collection::load_from_file()?;
    //process_args(&mut collection, get_args())?;
    //collection.flush_to_file()?;

    //ui::render(collection);
    gui::render(collection);

    Ok(())
}
