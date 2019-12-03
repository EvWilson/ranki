use std::error;

mod collection;
mod ui;

use clap::{clap_app, App};

fn main() -> Result<(), Box<dyn error::Error>> {
    let collection = collection::Collection::load_from_file()?;
    //process_args(&mut collection, get_args())?;
    //collection.flush_to_file()?;

    ui::render(collection);

    Ok(())
}

fn get_args() -> App<'static, 'static> {
    clap_app!(anki_rust =>
        (version: "0.1.0")
        (about: "A Rust port of the memory aid Anki")
        (@subcommand deck =>
            (about: "Operate on the decks in your collection")
            (@group operation =>
                (@arg add_question: -a +takes_value "Add a card by question")
                (@arg delete_id: -d +takes_value "Delete a card by id")
                (@arg print: -p "Prints the cards in the deck")
            )
        )
        (@subcommand collection =>
            (about: "Operate on your collection")
            (@group operation =>
                (@arg add_deck: -a +takes_value "Add a deck by name")
                (@arg delete_deck: -d +takes_value "Delete a deck by name")
                (@arg print: -p "Print the entire collection")
            )
        )
        (@subcommand card =>
            (about: "Operate on the cards in a deck")
            (@group operation =>
                (@arg print: -p +takes_value "Print a card by id")
            )
        )
        (@subcommand quiz =>
            (about: "Begin a quiz on elements in your collection")
        )
    )
}

fn process_args(
    collection: &mut collection::Collection,
    args: App,
) -> Result<(), Box<dyn error::Error>> {
    let matches = args.get_matches();

    if let Some(matches) = matches.subcommand_matches("collection") {
        if let Some(deck_title) = matches.value_of("add_deck") {
            collection.add_deck(deck_title);
        }
        if let Some(delete_idx) = matches.value_of("delete_deck") {
            let delete_idx = delete_idx.parse::<usize>()?;
            collection.remove_deck_by_pos(delete_idx)?;
        }
        if matches.is_present("print") {
            collection.print();
        }
    }

    Ok(())
}
