use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<u32> = it.take(n).map(|s| s.parse().unwrap()).collect();

    let mut stack: Vec<usize> = Vec::with_capacity(n);
    let mut out = String::with_capacity(n * 7);

    for (i, &x) in a.iter().enumerate() {
        while stack.last().is_some_and(|&p| a[p - 1] >= x) {
            stack.pop();
        }
        let ans = stack.last().copied().unwrap_or(0);
        write!(out, "{ans} ").unwrap();
        stack.push(i + 1);
    }

    println!("{}", out.trim_end());
}
