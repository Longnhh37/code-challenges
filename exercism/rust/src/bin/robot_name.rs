use rand::{Rng, RngExt};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

const MAX_NAMES: u32 = 26 * 26 * 1000;

pub struct RobotFactory {
    used: HashSet<u32>,
}

#[derive(Clone)]
pub struct Robot {
    name: String,
    factory: Rc<RefCell<RobotFactory>>,
}

impl RobotFactory {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            used: HashSet::new(),
        }))
    }

    fn idx_to_name(idx: u32) -> String {
        let d = idx % 1000;
        let letters = idx / 1000;

        let c2 = (letters % 26) as u8;
        let c1 = (letters / 26) as u8;

        format!("{}{}{:03}", (b'A' + c1) as char, (b'A' + c2) as char, d)
    }

    fn next_idx<R: Rng>(&mut self, rng: &mut R) -> u32 {
        if self.used.len() as u32 >= MAX_NAMES {
            panic!();
        }

        loop {
            let idx = rng.random_range(0..MAX_NAMES);
            if !self.used.contains(&idx) {
                self.used.insert(idx);
                return idx;
            }
        }
    }

    pub fn new_robot<R: Rng>(
        factory: &Rc<RefCell<Self>>,
        rng: &mut R,
    ) -> Robot {
        let mut f = factory.borrow_mut();
        let idx = f.next_idx(rng);

        Robot {
            name: Self::idx_to_name(idx),
            factory: Rc::clone(factory),
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R:Rng>(&mut self, rng: &mut R) {
        let mut factory = self.factory.borrow_mut();
        let idx = factory.next_idx(rng);
        self.name = RobotFactory::idx_to_name(idx)
    }
}

fn main() {}
