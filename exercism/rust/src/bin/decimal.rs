use num_bigint::BigInt;
use num_traits::Signed;
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Decimal {
    sign: bool,
    digits: Vec<char>,
    scale: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Self> {
        if input.is_empty() {
            return None;
        }
        let mut chars = input.chars().peekable();

        let sign = match chars.peek() {
            Some('+') => {
                chars.next();
                true
            }
            Some('-') => {
                chars.next();
                false
            }
            _ => true,
        };

        let mut saw_dot = false;
        let mut digits = Vec::new();
        let mut scale = 0usize;
        let mut saw_digit = false;

        for c in chars {
            match c {
                '0'..='9' => {
                    saw_digit = true;
                    digits.push(c);
                    if saw_dot {
                        scale += 1;
                    }
                }
                '.' if !saw_dot => {
                    saw_dot = true;
                }
                _ => return None,
            }
        }

        if !saw_digit {
            return None;
        }

        Some(Self::normalize(sign, digits, scale))
    }

    fn normalize(sign: bool, mut digits: Vec<char>, mut scale: usize) -> Self {
        while digits.len() > 1 && *digits.first().unwrap() == '0' {
            digits.remove(0);
        }

        while scale > 0 && let Some(&c) = digits.last() && c == '0' {
            digits.pop();
            scale -= 1;
        }

        if digits.is_empty() || digits.iter().all(|&c| c == '0') {
            return Self {
                sign: true,
                digits: vec!['0'],
                scale: 0,
            };
        }

        Self {
            sign,
            digits,
            scale,
        }
    }

    fn to_scaled_bigint(&self, target_scale: usize) -> BigInt {
        let mut digits = self.digits.clone();

        if target_scale > self.scale {
            digits.extend(std::iter::repeat_n('0', target_scale - self.scale));
        }

        let magnitude: BigInt = digits
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();

        if self.sign {
            magnitude
        } else {
            -magnitude
        }
    }

    fn from_bigint(value: BigInt, scale: usize) -> Self {
        let sign = !value.is_negative();
        let digits = value
            .abs()
            .to_string()
            .chars()
            .collect::<Vec<_>>();

        Self::normalize(sign, digits, scale)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let scale = self.scale.max(other.scale);
        let a = self.to_scaled_bigint(scale);
        let b = other.to_scaled_bigint(scale);
        a.cmp(&b)
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let scale = self.scale.max(other.scale);
        let result = self.to_scaled_bigint(scale) + other.to_scaled_bigint(scale);
        Self::from_bigint(result, scale)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let scale = self.scale.max(other.scale);
        let result = self.to_scaled_bigint(scale) - other.to_scaled_bigint(scale);
        Self::from_bigint(result, scale)
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let a: BigInt = self.digits.iter().collect::<String>().parse().unwrap();
        let b: BigInt = other.digits.iter().collect::<String>().parse().unwrap();

        let a = if self.sign { a } else { -a };
        let b = if other.sign { b } else { -b };

        let result = a * b;
        let scale = self.scale + other.scale;

        Self::from_bigint(result, scale)
    }
}

fn main() {}
