use std::collections::hash_map::HashMap;

use crate::behaviours::{self, Behaviour, BehaviourTypes};
use crate::item::{Item, ItemData, ItemType};

pub type TItems = Vec<Item>;

pub type TCountMap = HashMap<ItemType, u32>;

pub struct Pot {
    capacity: usize,
    items: TItems,
    majority_item_type: Option<ItemType>,
    score: i32,
    count_map: TCountMap,
}

impl Pot {
    pub fn new() -> Pot {
        Pot {
            items: vec![],
            score: 0,
            capacity: 10,
            majority_item_type: None,
            count_map: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        let mut pot_score: i32 = 0;

        for index in 0..self.items.len() {
            let mut item = self.items.get(index).unwrap().clone();
            let item_score = self.process_item_behaviours(item.get_item_data());

            item.add_generation();
            item.set_score(item_score);
            pot_score += item_score;

            self.items[index] = item;
        }
        self.score = pot_score;
    }

    pub fn add(&mut self, item: Item) {
        if self.items.len() < self.capacity - 1 {
            self.items.push(item);

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
                    BehaviourTypes::Test1 => behaviours::TestBehaviour1::run(&args),
                    BehaviourTypes::Test2 => behaviours::TestBehaviour2::run(&args),
                }
            }
        }
        new_score
    }

    pub fn get_items(&self) -> &TItems {
        &self.items
    }

    pub fn get_score(&self) -> &i32 {
        &self.score
    }

    pub fn get_count_map(&self) -> &TCountMap {
        &self.count_map
    }

    pub fn get_majority_item_type(&self) -> &Option<ItemType> {
        &self.majority_item_type
    }
}
