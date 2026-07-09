// ccc18j2
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let yesterday = input.trim().to_string();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let today = input.trim().to_string();

    let result = yesterday
        .chars()
        .zip(today.chars())
        .filter(|&(y, t)| y == 'C' && t == 'C')
        .count();

    println!("{}", result);
}
