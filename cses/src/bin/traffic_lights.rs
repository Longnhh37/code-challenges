use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let x = it.next().unwrap();
    let n = it.next().unwrap();
    let lights: Vec<_> = it.take(n).collect();

    let mut pts = Vec::with_capacity(n + 2);
    pts.push(0);
    pts.extend_from_slice(&lights);
    pts.push(x);
    pts.sort_unstable();

    let m = pts.len();
    let mut prev: Vec<usize> = (0..m).map(|i| i.saturating_sub(1)).collect();
    let mut next: Vec<usize> = (0..m).map(|i| (i + 1).min(m - 1)).collect();

    let mut best = pts.windows(2).map(|w| w[1] - w[0]).max().unwrap();

    let mut ans = vec![0usize; n];
    for (i, &p) in lights.iter().enumerate().rev() {
        ans[i] = best;
        let idx = pts.binary_search(&p).unwrap();
        let (l, r) = (prev[idx], next[idx]);
        next[l] = r;
        prev[r] = l;
        best = best.max(pts[r] - pts[l]);
    }

    let out = ans
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    println!("{out}");
}
