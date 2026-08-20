use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    data: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.data)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut data = String::with_capacity(16);
        let mut unit = 1;

        while num > 0 {
            let mut value = num % 10;
            while value > 0 {
                match value {
                    1..=3 | 6..=8 => {
                        data.push(get_roman_char(1, unit));
                        value -= 1;
                    }
                    4 => {
                        data.push(get_roman_char(5, unit));
                        data.push(get_roman_char(1, unit));
                        break;
                    }
                    5 => {
                        data.push(get_roman_char(5, unit));
                        break;
                    }
                    9 => {
                        data.push(get_roman_char(1, unit * 10));
                        data.push(get_roman_char(1, unit));
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            unit *= 10;
            num /= 10;
        }

        Self { data: data.chars().rev().collect::<String>() }
    }
}

fn get_roman_char(value: u32, unit: u32) -> char {
    match (value, unit) {
        (1, 1) => 'I',
        (5, 1) => 'V',
        (1, 10) => 'X',
        (5, 10) => 'L',
        (1, 100) => 'C',
        (5, 100) => 'D',
        (1, 1000) => 'M',
        _ => unreachable!(),
    }
}

fn main() {}
