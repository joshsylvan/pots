use crate::behaviours::{get_behaviours, BehaviourTypes, TBehaviours};
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ItemType {
    Red,
    Green,
    Blue,
    Yellow,
    TestItem1,
    TestItem2,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Copy, Clone)]
pub struct Item {
    item_type: ItemType,
    score: i32,
    generation: u32,
    behaviours: TBehaviours,
}

pub type ItemData = (ItemType, i32, TBehaviours);

impl Item {
    pub fn new(item_type: ItemType) -> Item {
        Item {
            item_type,
            score: 0,
            generation: 0,
            behaviours: get_behaviours(&item_type),
        }
    }

    pub fn new_random() -> Item {
        let item_type = Item::get_random_type();
        Item {
            item_type,
            score: 0,
            generation: 0,
            behaviours: get_behaviours(&item_type),
        }
    }

    pub fn get_type(&self) -> &ItemType {
        &self.item_type
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }

    pub fn get_behaviours(&self) -> &TBehaviours {
        &self.behaviours
    }
    pub fn add_generation(&mut self) {
        self.generation += 1;
    }

    fn get_random_type() -> ItemType {
        let mut rng = thread_rng();
        match rng.gen_range(0, 4) {
            0 => ItemType::Red,
            1 => ItemType::Green,
            2 => ItemType::Blue,
            _ => ItemType::Yellow,
        }
    }

    pub fn get_item_data(&self) -> ItemData {
        (
            self.get_type().clone(),
            self.get_score(),
            self.get_behaviours().clone(),
        )
    }
}
