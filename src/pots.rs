struct Pots {
    pots: Vec<pot::Pot>,
}

impl Pots {
    fn _new(pot_count: u32) -> Pots {
        let mut pots = vec![];

        for _ in 0..pot_count {
            pots.push(pot::Pot::new());
        }

        Pots { pots }
    }

    fn _add_pot(&mut self, pot: pot::Pot) {
        self.pots.push(pot);
    }
}
