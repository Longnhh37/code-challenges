use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let n = it.next().unwrap() as usize;
    let cost: Vec<u32> = std::iter::once(0).chain(it.take(n)).collect();

    println!("{}", solve(n, &cost));
}

fn solve(n: usize, cost: &[u32]) -> u32 {
    // dist[pos][last]
    let mut dist = vec![vec![u32::MAX; n]; n + 1];
    let mut heap = BinaryHeap::new();

    dist[1][0] = 0;
    heap.push(Reverse((0u32, 1usize, 0usize)));

    while let Some(Reverse((d, pos, last))) = heap.pop() {
        if d > dist[pos][last] {
            continue;
        }
        if pos == n {
            return d;
        }
        let forward = (pos + last < n).then(|| (pos + last + 1, last + 1));
        let backward = (last >= 1 && pos > last).then(|| (pos - last, last));

        for (np, nl) in [forward, backward].into_iter().flatten() {
            let nd = d + cost[np];
            if nd < dist[np][nl] {
                dist[np][nl] = nd;
                heap.push(Reverse((nd, np, nl)));
            }
        }
    }

    u32::MAX
}
