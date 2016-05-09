mod game_data;

use std::env;

fn main() {
    if std::env::args().len() != 2 {
        println!("Argument count is not 2!");
        std::process::exit(1);
    }
    let mut new_game_data = game_data::GameData::new();

    match new_game_data.load_from_file(&env::args().nth(1).unwrap()) {
        Ok(_) => println!("Loaded game!"),
        Err(m) => {
            println!("Error loading game: {}", m);
            return;
        }
    };
}
