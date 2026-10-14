use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let q = it.next().unwrap();

    let teleports: Vec<usize> = it.by_ref().take(n).collect();
    let mut adj = vec![Vec::new(); n];

    for (i, &t) in teleports.iter().enumerate() {


    }
}
