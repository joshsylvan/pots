use std::io;

extern crate pots;
use pots::errors::PotsInputError;
use pots::game_manager::GameManager;

fn main() -> io::Result<()> {
    println!("Pots Game");

    let mut game = GameManager::new();
    game.start_game();
    game.next_turn();

    loop {
        let selected_index;
        let selected_type;
        loop {
            println!("{}", game.container());
            println!("Current items: {}", game.display_current_items());
            println!("Select and item...");

            selected_index = match get_valid_index(get_input()?.trim(), game.current_items().len())
            {
                Ok(i) => i,
                Err(e) => {
                    print_and_clear(&e.to_string());
                    continue;
                }
            };

            selected_type = game
                .current_items()
                .get(selected_index)
                .unwrap()
                .get_type()
                .clone();

            print_and_clear(&format!("You have selected {}", selected_type));

            break;
        }

        loop {
            println!("{}\n", game.container());
            println!("What pot do you want to place {} item?", selected_type);

            let selected_pot =
                match get_valid_index(get_input()?.trim(), game.container().get_pots().len()) {
                    Ok(i) => i,
                    Err(e) => {
                        print_and_clear(&e.to_string());
                        continue;
                    }
                };

            let item = game.current_items_mut().remove(selected_index);
            game.container_mut().add_item_to_pot(item, selected_pot);

            print_and_clear(&format!(
                "You placed {} in pot {}\n",
                selected_type, selected_pot
            ));
            break;
        }

        if game.current_items().len() == 0 {
            print_and_clear("Updating stats...");
            game.container_mut().update();
            println!("Next turn...");
            game.next_turn();
        } else {
            println!("{}", game.container());
        }
    }
}

fn get_valid_index(input: &str, max_value: usize) -> Result<usize, PotsInputError> {
    let player_input = input.parse::<usize>();
    let parsed_index = match player_input {
        Ok(i) => i,
        Err(_) => return Err(PotsInputError),
    };

    if parsed_index >= max_value {
        return Err(PotsInputError);
    }

    Ok(parsed_index)
}

fn print_and_clear(output: &str) {
    print!("\x1B[2J");
    println!("{}", output);
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(error) => Err(error),
    }
}
