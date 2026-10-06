use std::collections::HashSet;
use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let nums: Vec<_> = it.take(n).collect();

    let mut seen = HashSet::new();

    for &n1 in &nums {
        let temp: Vec<_> = seen.clone().into_iter().collect();
        for n2 in temp {
            seen.insert(n1 + n2);
        }
        seen.insert(n1);
    }

    let mut seen: Vec<_> = seen.into_iter().collect();
    seen.sort_unstable();

    let mut out = String::with_capacity(seen.len() * 7);
    for &n in &seen {
        let _ = write!(out, "{} ", n);
    }
    println!("{}", seen.len());
    println!("{}", out);
}
