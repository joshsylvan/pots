use std::collections::hash_map::HashMap;

use crate::behaviours::{self, Behaviour, BehaviourTypes};
use crate::item::{Item, ItemData, ItemType};

const MAX_POT_SIZE: usize = 10;

pub type TItems = [Option<Item>; MAX_POT_SIZE];

pub type TCountMap = HashMap<ItemType, u32>;

pub struct Pot {
    current_size: usize,
    items: TItems,
    population: i32,
    majority_item_type: Option<ItemType>,
    score: i32,
    count_map: TCountMap,
}

impl Pot {
    pub fn new() -> Pot {
        Pot {
            current_size: 0,
            items: [None; MAX_POT_SIZE],
            population: 0,
            score: 0,
            majority_item_type: None,
            count_map: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        let length = self.items.len();
        let mut pot_score: i32 = 0;

        for index in 0..length {
            if let Some(item_ref) = &self.items[index] {
                let mut item = item_ref.clone();
                let item_score = self.process_item_behaviours(item.get_item_data());
                item.add_generation();
                item.set_score(item_score);
                pot_score += item_score;
                self.items[index] = Some(item);
            }
        }

        self.score += pot_score;
    }

    pub fn add(&mut self, item: Item) {
        if self.current_size < self.items.len() - 1 {
            self.items[self.current_size] = Some(item);
            self.current_size += 1;
            self.population += 1;

            self.count_map
                .entry(item.get_type().clone())
                .and_modify(|v| *v += 1)
                .or_insert(1);

            // Work out new majority
            if self.majority_item_type == None {
                self.majority_item_type = Some(item.get_type().clone());
            } else {
                let current_item_count = *self.count_map.get(item.get_type()).unwrap();
                let majority_count = *self
                    .count_map
                    .get(&self.majority_item_type.unwrap())
                    .unwrap();
                if current_item_count > majority_count {
                    self.majority_item_type = Some(item.get_type().clone());
                }
            }
        }
    }

    fn process_item_behaviours(&self, item_data: ItemData) -> i32 {
        let (item_type, _, behaviours) = item_data;

        let mut new_score: i32 = 0;
        for behaviour in behaviours.iter() {
            let args = (
                &item_type,
                &self.items,
                &self.count_map,
                &self.majority_item_type,
            );
            if let Some(b) = behaviour {
                new_score += match b {
                    BehaviourTypes::Happy => behaviours::Happy::run(&args),
                    BehaviourTypes::Loner => behaviours::Loner::run(&args),
                    BehaviourTypes::Majority => behaviours::Majority::run(&args),
                    BehaviourTypes::Minority => behaviours::Minority::run(&args),
                    BehaviourTypes::Neutral => behaviours::Neutral::run(&args),
                    BehaviourTypes::Social => behaviours::Social::run(&args),
                }
            }
        }

        new_score
    }

    pub fn items(&self) -> &TItems {
        &self.items
    }

    pub fn score(&self) -> &i32 {
        &self.score
    }

    pub fn majority_item_type(&self) -> &Option<ItemType> {
        &self.majority_item_type
    }
}
