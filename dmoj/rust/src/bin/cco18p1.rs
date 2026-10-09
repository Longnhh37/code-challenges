use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let geese: Vec<u8> = it.next().unwrap().bytes().collect();
    let gscore: Vec<usize> = (0..n)
        .map(|_| it.next().unwrap().parse::<usize>().unwrap())
        .collect();
    let hawk: Vec<u8> = it.next().unwrap().bytes().collect();
    let hscore: Vec<usize> = (0..n)
        .map(|_| it.next().unwrap().parse::<usize>().unwrap())
        .collect();

    let mut dp = vec![vec![0; n + 1]; n + 1];
}
