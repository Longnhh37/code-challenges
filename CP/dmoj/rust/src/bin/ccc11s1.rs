//ccc11s1

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.to_ascii_lowercase();

    let s = input.chars().filter(|&x| x == 's').count();
    let t = input.chars().filter(|&x| x == 't').count();

    println!("{}", if t > s { "English" } else { "French" });
}
