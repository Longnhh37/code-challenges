use rand::RngExt;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

fn gen_name<R: rand::Rng>(rng: &mut R) -> String {
    let c1 = (b'A' + rng.random_range(0u8..26)) as char;
    let c2 = (b'A' + rng.random_range(0u8..26)) as char;
    let num: u32 = rng.random_range(0..1000);
    format!("{}{}{:03}", c1, c2, num)
}

pub struct RobotFactory {
    used: Rc<RefCell<HashSet<String>>>,
}

pub struct Robot {
    name: String,
    used: Rc<RefCell<HashSet<String>>>,
}

#[allow(clippy::new_without_default)]
impl RobotFactory {
    pub fn new() -> Self {
        Self {
            used: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: rand::Rng>(&mut self, rng: &mut R) -> Robot {
        loop {
            let name = gen_name(rng);
            if self.used.borrow_mut().insert(name.clone()) {
                return Robot {
                    name,
                    used: Rc::clone(&self.used),
                };
            }
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: rand::Rng>(&mut self, rng: &mut R) {
        let old_name = self.name.clone();

        let new_name = loop {
            let name = gen_name(rng);
            if self.used.borrow_mut().insert(name.clone()) {
                break name;
            }
        };

        self.used.borrow_mut().remove(&old_name);

        self.name = new_name;
    }
}
fn main() {}
