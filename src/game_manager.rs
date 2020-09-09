use rand::{thread_rng, Rng};

use crate::container::Container;
use crate::item::Item;

const _MAX_POTS: usize = 10;
const MAX_NEW_ITEMS: u32 = 5;
const MIN_NEW_ITEMS: u32 = 2;

pub struct GameManager {
    container: Container,
    is_game_over: bool,
    current_items: Vec<Item>,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            container: Container::new(3),
            is_game_over: true,
            current_items: vec![],
        }
    }

    pub fn start_game(&mut self) {
        self.is_game_over = false;
    }

    pub fn next_turn(&mut self) {
        let mut rng = thread_rng();
        for _ in 0..rng.gen_range(MIN_NEW_ITEMS, MAX_NEW_ITEMS) {
            self.current_items.push(Item::new_random());
        }
    }

    pub fn container(&self) -> &Container {
        &self.container
    }

    pub fn container_mut(&mut self) -> &mut Container {
        &mut self.container
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
