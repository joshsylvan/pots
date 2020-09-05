mod behaviours;
mod item;
mod pot;

use crate::item::Item;
use crate::pot::Pot;

const _MAX_POTS: usize = 10;

fn main() {
    println!("Pots Game");

    let mut pot = Pot::new();

    for _ in 0..10 {
        pot.add(Item::new_random());
    }

    for item in pot.items().iter() {
        if let Some(i) = item {
            println!("{:?}", i.get_type());
        }
    }

    pot.update();

    println!("Pot score: {}", pot.score());
    println!("Pot majority item: {:?}", pot.majority_item_type());
}
