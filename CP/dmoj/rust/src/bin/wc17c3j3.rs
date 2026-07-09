// wc17c3j3
use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    let text = text.trim();

    let mut lower = 0;
    let mut upper = 0;
    let mut digit = 0;

    for ch in text.chars() {
        if ch.is_ascii_lowercase() {
            lower += 1;
        } else if ch.is_ascii_uppercase() {
            upper += 1;
        } else if ch.is_ascii_digit() {
            digit += 1;
        }
    }

    if text.len() >= 8 && text.len() <= 12 && lower >= 3 && upper >= 2 && digit >= 1 {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}
