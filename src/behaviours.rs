use crate::item::ItemType;
use crate::pot::{TCountMap, TItems};

#[derive(Copy, Clone)]
pub enum BehaviourTypes {
    Happy,
    Loner,
    Majority,
    Minority,
    Neutral,
    Social,
}

pub type TBehaviourRunArgs<'a> = (
    &'a ItemType,
    &'a TItems,
    &'a TCountMap,
    &'a Option<ItemType>,
);

pub trait Behaviour {
    fn run(
        // item_type: &ItemType,
        // items: &TItems,
        // count_map: &TCountMap,
        // majority_item_type: &Option<ItemType>,
        (_item_type, _items, _count_map, _majority_item_type): &TBehaviourRunArgs,
    ) -> i32 {
        0
    }
}

pub struct Happy;
impl Behaviour for Happy {
    fn run((_item_type, _items, _count_map, _majority_item_type): &TBehaviourRunArgs) -> i32 {
        1
    }
}

pub struct Loner;
impl Behaviour for Loner {
    fn run((item_type, _items, count_map, _majority_item_type): &TBehaviourRunArgs) -> i32 {
        let val = count_map.get(item_type).unwrap_or_else(|| &1u32);
        if *val == 1 {
            return 1;
        }
        -1
    }
}

pub struct Neutral;
impl Behaviour for Neutral {
    fn run((_item_type, _items, _count_map, _majority_item_type): &TBehaviourRunArgs) -> i32 {
        0
    }
}

pub struct Social;
impl Behaviour for Social {
    fn run((item_type, _items, count_map, _majority_item_type): &TBehaviourRunArgs) -> i32 {
        let val = count_map.get(item_type).unwrap_or_else(|| &1u32);
        if *val > 1 {
            return 1;
        }
        -1
    }
}

pub struct Minority;
impl Behaviour for Minority {
    fn run((item_type, _items, _count_map, majority_item_type): &TBehaviourRunArgs) -> i32 {
        if let Some(majority) = majority_item_type {
            if *item_type == majority {
                return -1;
            }
        }
        1
    }
}

pub struct Majority;
impl Behaviour for Majority {
    fn run((item_type, _items, _count_map, majority_item_type): &TBehaviourRunArgs) -> i32 {
        if let Some(majority) = majority_item_type {
            if *item_type == majority {
                return 1;
            }
        }
        -1
    }
}
