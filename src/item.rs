use crate::behaviours::BehaviourTypes;
use rand::{thread_rng, Rng};

const MAX_BEHAVIOUR_SIZE: usize = 3;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ItemType {
    Red,
    Green,
    Blue,
    Yellow,
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
    pub fn _new(item_type: Option<ItemType>) -> Item {
        if let Some(t) = item_type {
            Item {
                item_type: t,
                score: 0,
                generation: 0,
                behaviours: Item::generate_behaviours(&t),
            }
        } else {
            Item {
                item_type: ItemType::Red,
                score: 0,
                generation: 0,
                behaviours: [None; MAX_BEHAVIOUR_SIZE],
            }
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

    pub fn new_random() -> Item {
        let item_type = Item::get_random_type();
        Item {
            item_type,
            score: 0,
            generation: 0,
            behaviours: Item::generate_behaviours(&item_type),
        }
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
            ItemType::Red => [
                Some(BehaviourTypes::Loner),
                Some(BehaviourTypes::Minority),
                None,
            ],
            ItemType::Green => [
                Some(BehaviourTypes::Social),
                Some(BehaviourTypes::Majority),
                None,
            ],
            ItemType::Blue => [Some(BehaviourTypes::Neutral), None, None],
            ItemType::Yellow => [Some(BehaviourTypes::Happy), None, None],
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
