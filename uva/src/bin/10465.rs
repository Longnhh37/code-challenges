use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let mut out = BufWriter::new(io::stdout().lock());

    while let (Some(m), Some(n), Some(t)) = (it.next(), it.next(), it.next()) {
        let mut dp: Vec<Option<u32>> = vec![None; t + 1];
        dp[0] = Some(0);

        for i in 1..=t {
            let a = i.checked_sub(m).and_then(|j| dp[j]);
            let b = i.checked_sub(n).and_then(|j| dp[j]);
            dp[i] = a.max(b).map(|c| c + 1);
        }

        let (used, burgers) = (0..=t).rev().find_map(|i| dp[i].map(|c| (i, c))).unwrap();

        let beer = t - used;
        if beer == 0 {
            writeln!(out, "{burgers}").unwrap();
        } else {
            writeln!(out, "{burgers} {beer}").unwrap();
        }

        out.flush().unwrap();
    }
}
