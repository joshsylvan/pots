use crate::item::ItemType;
use crate::pot::{TCountMap, TItems};

const MAX_BEHAVIOUR_SIZE: usize = 3;

#[derive(Copy, Clone)]
pub enum BehaviourTypes {
    Happy,
    Loner,
    Majority,
    Minority,
    Neutral,
    Social,
    Test1,
    Test2,
}

pub type TBehaviours = [Option<BehaviourTypes>; MAX_BEHAVIOUR_SIZE];

pub fn get_behaviours(item_type: &ItemType) -> TBehaviours {
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

pub type TBehaviourRunArgs<'a> = (
    &'a ItemType,
    &'a TItems,
    &'a TCountMap,
    &'a Option<ItemType>,
);

pub trait Behaviour {
    fn run((_item_type, _items, _count_map, _majority_item_type): &TBehaviourRunArgs) -> i32 {
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
impl Behaviour for Neutral {}

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

pub struct TestBehaviour1;
impl Behaviour for TestBehaviour1 {
    /// Checks if it is the only one of it's type
    fn run((item_type, _items, count_map, _majority_item_type): &TBehaviourRunArgs) -> i32 {
        let val = count_map.get(item_type).unwrap_or_else(|| &1u32);
        if *val == 1 {
            return 1;
        }
        -1
    }
}

pub struct TestBehaviour2;
impl Behaviour for TestBehaviour2 {
    /// Checks if it is with any similar types
    fn run((item_type, _items, count_map, _majority_item_type): &TBehaviourRunArgs) -> i32 {
        let val = count_map.get(item_type).unwrap_or_else(|| &1u32);
        if *val == 1 {
            return 1;
        }
        -1
    }
}
