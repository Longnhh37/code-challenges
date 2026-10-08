use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();

    let a: Vec<_> = it.take(n).collect();
}
