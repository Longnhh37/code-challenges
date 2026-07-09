#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score: score & 0xFF,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & (*allergen as u32)) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut out = Vec::with_capacity(8);

        for &a in &ALLERGENS {
            if (self.score & (a as u32)) != 0 {
                out.push(a);
            }
        }

        out
    }
}
