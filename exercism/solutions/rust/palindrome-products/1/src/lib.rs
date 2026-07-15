use std::collections::HashSet;

pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for u64 {
    fn is_palindrome(&self) -> bool {
        let mut x = *self;
        let mut rev = 0;

        while x > 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }

        rev == *self
    }
}

#[derive(Debug)]
pub struct PalindromeProduct {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl PalindromeProduct {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(from: u64, to: u64) -> Option<(PalindromeProduct, PalindromeProduct)> {
    if from >= to {
        return None;
    }

    let mut min_val = u64::MAX;
    let mut max_val = 0;

    let mut min_factors = HashSet::new();
    let mut max_factors = HashSet::new();

    for i in from..=to {
        for j in (i..=to).rev() {
            let prod = i * j;

            if prod < max_val {
                break;
            }

            if !prod.is_palindrome() {
                continue;
            }

            let pair = (i, j);

            if prod > max_val {
                max_val = prod;
                max_factors.clear();
                max_factors.insert(pair);
            } else if prod == max_val {
                max_factors.insert(pair);
            }

        }
    }

    for i in from..=to {
        for j in i..=to {
            let prod = i * j;

            if prod > min_val {
                break;
            }

            if !prod.is_palindrome() {
                continue;
            }

            let pair = (i, j);

            if prod < min_val {
                min_val = prod;
                min_factors.clear();
                min_factors.insert(pair);
            } else if prod == min_val {
                min_factors.insert(pair);
            }
        }
    }

    if min_factors.is_empty() {
        None
    } else {
        Some((
            PalindromeProduct {
                value: min_val,
                factors: min_factors,
            },
            PalindromeProduct {
                value: max_val,
                factors: max_factors,
            },
        ))
    }
}
