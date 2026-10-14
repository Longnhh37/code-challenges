use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const START: usize = 0;
const INF: u64 = u64::MAX;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();

    let mut adj = vec![Vec::new(); n];
    for _ in 0..it.next().unwrap() {
        let u = it.next().unwrap() - 1;
        let v = it.next().unwrap() - 1;
        let w = it.next().unwrap() as u64;
        adj[u].push((v, w));
    }

    let mut dist = vec![INF; n];
    let mut ways = vec![INF; n];
    let mut min_f = vec![INF; n];
    let mut max_f = vec![INF; n];
    dist[START] = 0;
    ways[START] = 1;
    min_f[START] = 0;
    max_f[START] = 0;

    let mut heap = BinaryHeap::from([Reverse((0u64, START))]);
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            match nd.cmp(&dist[v]) {
                Ordering::Less => {
                    dist[v] = nd;
                    ways[v] = ways[u];
                    min_f[v] = min_f[u] + 1;
                    max_f[v] = max_f[u] + 1;
                    heap.push(Reverse((nd, v)));
                }
                Ordering::Equal => {
                    ways[v] = (ways[v] + ways[u]) % MOD;
                    min_f[v] = min_f[v].min(min_f[u] + 1);
                    max_f[v] = max_f[v].max(max_f[u] + 1);
                }
                Ordering::Greater => {}
            }
        }
    }

    println!(
        "{} {} {} {}",
        dist[n - 1],
        ways[n - 1],
        min_f[n - 1],
        max_f[n - 1]
    );
}
