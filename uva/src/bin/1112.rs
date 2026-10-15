use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let cases = it.next().unwrap();

    for case in 0..cases {
        let n = it.next().unwrap();
        let exit = it.next().unwrap() - 1;
        let limit = it.next().unwrap() as u64;

        let mut radj = vec![Vec::new(); n];
        for _ in 0..it.next().unwrap() {
            let u = it.next().unwrap() - 1;
            let v = it.next().unwrap() - 1;
            let w = it.next().unwrap() as u64;
            radj[v].push((u, w));
        }

        let mut dist = vec![u64::MAX; n];
        dist[exit] = 0;
        let mut heap = BinaryHeap::from([Reverse((0u64, exit))]);

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &radj[u] {
                let nd = w + d;
                if nd <= limit && nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }

        let cnt = dist.iter().filter(|&&d| d <= limit).count();

        if case > 0 {
            println!()
        }
        println!("{cnt}");
    }
}
