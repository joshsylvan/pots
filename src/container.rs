use std::fmt;

use crate::item::Item;
use crate::pot;

pub struct Container {
    container: Vec<pot::Pot>,
    global_score: i32,
}

impl Container {
    pub fn new(pot_count: u32) -> Container {
        let mut container = vec![];

        for _ in 0..pot_count {
            container.push(pot::Pot::new());
        }

        Container {
            container,
            global_score: 0,
        }
    }

    fn _add_pot(&mut self, pot: pot::Pot) {
        self.container.push(pot);
    }

    pub fn add_new_pot(&mut self) {
        self.container.push(pot::Pot::new());
    }

    pub fn get_pots(&self) -> &Vec<pot::Pot> {
        &self.container
    }

    pub fn add_item_to_pot(&mut self, item: Item, pot_index: usize) {
        self.container.get_mut(pot_index).unwrap().add(item);
    }

    pub fn update(&mut self) {
        self.global_score = 0;
        for pot in &mut self.container {
            pot.update();
            self.global_score += pot.get_score();
        }
    }

    pub fn get_score(&self) -> &i32 {
        &self.global_score
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("Current score: {}\n\n", self.global_score);
        let mut index: u32 = 0;
        for pot in &self.container {
            output += &format!("Pot {}: ", index);

            for item in pot.get_items() {
                output += &format!("{} ", &item.get_type());
            }

            index += 1;
            output += "\n";
        }
        write!(f, "{}", output)
    }
}
