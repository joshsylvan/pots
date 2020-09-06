use rand::{thread_rng, Rng};
use std::io;

extern crate pots;
use pots::errors::PotsInputError;
use pots::item::Item;
use pots::pots::Pots;

const _MAX_POTS: usize = 10;
const MAX_NEW_ITEMS: u32 = 5;
const MIN_NEW_ITEMS: u32 = 2;

struct GameManager {
    pots: Pots,
    is_game_over: bool,
    current_items: Vec<Item>,
}

impl GameManager {
    fn new() -> GameManager {
        GameManager {
            pots: Pots::new(3),
            is_game_over: true,
            current_items: vec![],
        }
    }

    fn start_game(&mut self) {
        self.is_game_over = false;
    }

    fn next_turn(&mut self) {
        let mut rng = thread_rng();
        for _ in 0..rng.gen_range(MIN_NEW_ITEMS, MAX_NEW_ITEMS) {
            self.current_items.push(Item::new_random());
        }
    }

    fn pots(&self) -> &Pots {
        &self.pots
    }

    fn pots_mut(&mut self) -> &mut Pots {
        &mut self.pots
    }

    pub fn current_items(&self) -> &Vec<Item> {
        &self.current_items
    }

    pub fn current_items_mut(&mut self) -> &mut Vec<Item> {
        &mut self.current_items
    }

    pub fn display_current_items(&self) -> String {
        let mut display = String::new();
        let mut index = 0;
        for item in &self.current_items {
            display += &format!("({}){} ", index, item.get_type());
            index += 1;
        }

        display
    }
}

fn main() -> io::Result<()> {
    println!("Pots Game");

    let mut game = GameManager::new();
    game.start_game();
    game.next_turn();

    loop {
        let selected_index;
        let selected_type;
        loop {
            println!("{}", game.pots());
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
            println!("{}\n", game.pots());
            println!("What pot do you want to place {} item?", selected_type);

            let selected_pot =
                match get_valid_index(get_input()?.trim(), game.pots().get_pots().len()) {
                    Ok(i) => i,
                    Err(e) => {
                        print_and_clear(&e.to_string());
                        continue;
                    }
                };

            let item = game.current_items_mut().remove(selected_index);
            game.pots_mut().add_item_to_pot(item, selected_pot);

            print_and_clear(&format!(
                "You placed {} in pot {}\n",
                selected_type, selected_pot
            ));
            break;
        }

        if game.current_items().len() == 0 {
            print_and_clear("Updating stats...");
            game.pots_mut().update();
            println!("Next turn...");
            game.next_turn();
        } else {
            println!("{}", game.pots());
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
