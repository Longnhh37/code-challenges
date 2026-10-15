use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, Read},
};

const MOD: u64 = 1_000_000;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let n = it.next().unwrap() as usize;

    let mut adj = vec![Vec::new(); n];
    for u in 0..n {
        for v in 0..n {
            let w = it.next().unwrap();
            if w != 0 {
                adj[u].push((v, w));
            }
        }
    }

    let m = it.next().unwrap() as usize;
    let mut has_cookies = vec![false; n];
    for t in it.take(m) {
        has_cookies[t as usize - 1] = true;
    }

    let mut dist = vec![u64::MAX; 2 * n];
    let mut ways = vec![0_u64; 2 * n];

    let start = has_cookies[0] as usize;
    dist[start] = 0;
    ways[start] = 1;
    let mut heap = BinaryHeap::from([Reverse((0u64, start))]);

    while let Some(Reverse((d, s))) = heap.pop() {
        if d > dist[s] {
            continue;
        }
        let (u, flag) = (s / 2, s % 2);
        for &(v, w) in &adj[u] {
            let t = v * 2 + (flag == 1 || has_cookies[v]) as usize;
            let nd = d + w;
            if nd < dist[t] {
                dist[t] = nd;
                ways[t] = ways[s];
                heap.push(Reverse((nd, t)));
            } else if nd == dist[t] {
                ways[t] = (ways[t] + ways[s]) % MOD;
            }
        }
    }

    let end = (n - 1) * 2 + 1;
    println!("{} {}", dist[end], ways[end] % MOD);
}
