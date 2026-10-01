use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let n = it.next().unwrap();
    let mut out = BufWriter::new(io::stdout().lock());

    for _ in 0..n {
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        writeln!(out, "{}", if can_empty(a, b) { "YES" } else { "NO" }).unwrap();
    }
}

fn can_empty(a: u32, b: u32) -> bool {
    (a + b) % 3 == 0 && a.max(b) <= 2 * a.min(b)
}
