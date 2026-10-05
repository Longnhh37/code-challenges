use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let n = it.next().unwrap() as usize;
    let nums: HashSet<_> = it.take(n).collect();
    println!("{}", nums.len());
}
