use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const INF: u64 = u64::MAX;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap();

    let mut adj = vec![Vec::new(); n];
    for _ in 0..m {
        let u = it.next().unwrap() as usize - 1;
        let v = it.next().unwrap() as usize - 1;
        let w = it.next().unwrap();
        adj[u].push((v, w));
    }

    let mut dist: Vec<u64> = vec![INF; n];
    dist[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u64, 0usize)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    let out = dist
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", out);
}
