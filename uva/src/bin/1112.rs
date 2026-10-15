use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let tests = it.next().unwrap();

    for _ in 0..tests {
        let n = it.next().unwrap();
        let exit = it.next().unwrap() - 1;
        let limit = it.next().unwrap() as u64;

        let mut adj = vec![Vec::new(); n];
        for _ in 0..it.next().unwrap() {
            let u = it.next().unwrap();
            let v = it.next().unwrap();
            let w = it.next().unwrap() as u64;
            adj[u].push((v, w));
        }

        let mut cnt = 0;
        'outer: for start in 0..n {
            let mut heap = BinaryHeap::from([Reverse((0u64, start))]);
            let mut dist = vec![u64::MAX; n];
            dist[start] = 0;

            while let Some(Reverse((d, u))) = heap.pop() {
                if d > dist[u] {
                    continue;
                }
                for &(v, w) in &adj[u] {
                    let nd = w + d;
                    if nd > limit {
                        continue 'outer;
                    }
                    if v == exit {
                        cnt += 1;
                        continue 'outer;
                    }
                    if nd < dist[v] {
                        dist[v] = nd;
                        heap.push(Reverse((nd, v)));
                    }
                }
            }
        }

        println!("{cnt}");
    }
}
