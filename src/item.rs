use crate::behaviours::BehaviourTypes;
use rand::{thread_rng, Rng};
use std::fmt;

const MAX_BEHAVIOUR_SIZE: usize = 3;

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
    behaviours: [Option<BehaviourTypes>; MAX_BEHAVIOUR_SIZE],
}

pub type ItemData = (ItemType, i32, [Option<BehaviourTypes>; MAX_BEHAVIOUR_SIZE]);

impl Item {
    pub fn new(item_type: ItemType) -> Item {
        Item {
            item_type,
            score: 0,
            generation: 0,
            behaviours: Item::generate_behaviours(&item_type),
        }
    }

    pub fn new_random() -> Item {
        let item_type = Item::get_random_type();
        Item {
            item_type,
            score: 0,
            generation: 0,
            behaviours: Item::generate_behaviours(&item_type),
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

    pub fn get_behaviours(&self) -> &[Option<BehaviourTypes>; MAX_BEHAVIOUR_SIZE] {
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

    fn generate_behaviours(item_type: &ItemType) -> [Option<BehaviourTypes>; MAX_BEHAVIOUR_SIZE] {
        match item_type {
            ItemType::Red => [Some(BehaviourTypes::Loner), None, None],
            ItemType::Green => [
                Some(BehaviourTypes::Social),
                Some(BehaviourTypes::Majority),
                None,
            ],
            ItemType::Blue => [Some(BehaviourTypes::Neutral), None, None],
            ItemType::Yellow => [Some(BehaviourTypes::Happy), None, None],
            ItemType::TestItem1 => [Some(BehaviourTypes::Loner), None, None],
            ItemType::TestItem2 => [Some(BehaviourTypes::Social), None, None],
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
