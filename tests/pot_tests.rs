extern crate pots;
use pots::item::{Item, ItemType};
use pots::pot::Pot;

fn add_items_to_pot(pot: Pot, item_1_count: u32, item_2_count: u32) -> Pot {
    let mut pot = pot;
    for _ in 0..item_1_count {
        pot.add(Item::new(ItemType::TestItem1));
    }
    for _ in 0..item_2_count {
        pot.add(Item::new(ItemType::TestItem2));
    }

    pot
}

#[test]
fn it_adds_item_to_pot() {
    let mut pot = Pot::new();

    assert_eq!(pot.get_items().len(), 0);

    let item = Item::new(ItemType::Blue);

    pot.add(item.clone());

    assert_eq!(pot.get_items().len(), 1);
    assert_eq!(pot.get_items().get(0).unwrap().get_type(), item.get_type());
    assert_eq!(*pot.get_count_map().get(&ItemType::Blue).unwrap(), 1);
}

#[test]
fn it_adds_items_correctly_to_count_map() {
    let mut pot = Pot::new();

    assert_eq!(pot.get_count_map().len(), 0);

    let item_1_count = 3;
    let item_2_count = 2;

    pot = add_items_to_pot(pot, item_1_count, item_2_count);

    assert_eq!(
        *pot.get_count_map().get(&ItemType::TestItem1).unwrap(),
        item_1_count
    );
    assert_eq!(
        *pot.get_count_map().get(&ItemType::TestItem2).unwrap(),
        item_2_count
    );
}

#[test]
fn it_should_calculate_majority_type() {
    let mut pot = Pot::new();

    assert_eq!(*pot.get_majority_item_type(), None);

    pot = add_items_to_pot(pot, 1, 0);
    assert_eq!(*pot.get_majority_item_type(), Some(ItemType::TestItem1));

    pot = add_items_to_pot(pot, 0, 1);
    assert_eq!(*pot.get_majority_item_type(), Some(ItemType::TestItem1));

    pot = add_items_to_pot(pot, 0, 1);
    assert_eq!(*pot.get_majority_item_type(), Some(ItemType::TestItem2));

    pot = add_items_to_pot(pot, 3, 0);
    assert_eq!(*pot.get_majority_item_type(), Some(ItemType::TestItem1));
}

#[test]
fn test_process_item_behaviours() {
    let mut pot = Pot::new();

    pot.add(Item::new(ItemType::TestItem1));
    pot.update();
    assert_eq!(pot.get_items().len(), 1);
    assert_eq!(*pot.get_score(), 1);

    pot.add(Item::new(ItemType::TestItem1));
    pot.update();
    assert_eq!(*pot.get_score(), -2);
    assert_eq!(pot.get_items()[0].get_score(), -1);
}

#[test]
fn it_calculates_score() {
    let mut pot = Pot::new();

    assert_eq!(*pot.get_score(), 0);

    pot = add_items_to_pot(pot, 2, 0);
    pot.update();
    assert_eq!(*pot.get_score(), -2);

    pot = add_items_to_pot(pot, 0, 0);
    pot.update();
    assert_eq!(*pot.get_score(), -2);

    pot = add_items_to_pot(pot, 0, 1);
    pot.update();
    assert_eq!(*pot.get_score(), -3);

    pot = add_items_to_pot(pot, 0, 1);
    pot.update();
    assert_eq!(*pot.get_score(), 0);
}
