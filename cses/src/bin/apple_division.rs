use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let p: Vec<_> = it.take(n).collect();
    let total: usize = p.iter().sum();

    let best = (0..1u32 << n)
        .map(|mask| {
            let s: usize = (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| p[i]).sum();
            total.abs_diff(2 * s)
        })
        .min()
        .unwrap();

    println!("{best}");
}
