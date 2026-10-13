use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_ascii_whitespace();
    let word = it.next().unwrap().as_bytes();
    let pat = it.next().unwrap().as_bytes();
}
