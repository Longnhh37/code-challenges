use std::io::{self, BufWriter, Read, Write};

const MAX_K: usize = 100;

fn to_cents(s: &str) -> i64 {
    (s.parse::<f64>().unwrap() * 100.0).round() as i64
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().filter(|l| !l.trim().is_empty());
    let mut out = BufWriter::new(io::stdout().lock());

    let mut case = 0;

    while let Some(header) = lines.next() {
        case += 1;

        let mut h = header.split_ascii_whitespace();
        let unit = to_cents(h.next().unwrap());
        let m: usize = h.next().unwrap().parse().unwrap();

        let mut combos: Vec<(usize, i64)> = vec![(1, unit)];
        for _ in 0..m {
            let mut parts = lines.next().unwrap().split_ascii_whitespace();
            let n: usize = parts.next().unwrap().parse().unwrap();
            let p = to_cents(parts.next().unwrap());
            combos.push((n, p));
        }

        let mut dp = vec![i64::MAX; MAX_K + 1];
        dp[0] = 0;
        for i in 1..=MAX_K {
            dp[i] = combos
                .iter()
                .map(|&(n, p)| dp[i.saturating_sub(n)] + p)
                .min()
                .unwrap();
        }

        writeln!(out, "Case {case}:").unwrap();

        for k in lines.next().unwrap().split_ascii_whitespace() {
            let k: usize = k.parse().unwrap();
            let c = dp[k];
            writeln!(out, "Buy {k} for ${}.{:02}", c / 100, c % 100).unwrap();
        }
    }

    out.flush().unwrap();
}
