use std::fmt;

use crate::item::Item;
use crate::pot;

pub struct Pots {
    pots: Vec<pot::Pot>,
    global_score: i32,
}

impl Pots {
    pub fn new(pot_count: u32) -> Pots {
        let mut pots = vec![];

        for _ in 0..pot_count {
            pots.push(pot::Pot::new());
        }

        Pots {
            pots,
            global_score: 0,
        }
    }

    fn _add_pot(&mut self, pot: pot::Pot) {
        self.pots.push(pot);
    }

    pub fn add_new_pot(&mut self) {
        self.pots.push(pot::Pot::new());
    }

    pub fn get_pots(&self) -> &Vec<pot::Pot> {
        &self.pots
    }

    pub fn add_item_to_pot(&mut self, item: Item, pot_index: usize) {
        self.pots.get_mut(pot_index).unwrap().add(item);
    }

    pub fn update(&mut self) {
        self.global_score = 0;
        for pot in &mut self.pots {
            pot.update();
            self.global_score += pot.get_score();
        }
    }

    pub fn get_score(&self) -> &i32 {
        &self.global_score
    }
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("Current score: {}\n\n", self.global_score);
        let mut index: u32 = 0;
        for pot in &self.pots {
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
