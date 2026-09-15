use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap());

    let t = it.next().unwrap();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..t {
        let n = it.next().unwrap();
        let sum: i64 = (0..n).map(|_| it.next().unwrap()).sum();

        let ans = if sum.rem_euclid(4) == 0 { "YES" } else { "NO" };
        writeln!(out, "{}", ans).unwrap();
    }
    out.flush().unwrap();
}
