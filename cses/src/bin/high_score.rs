use std::collections::BinaryHeap;
use std::io::Read;

const INF: i64 = i64::MIN;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());
    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;

    let mut adj = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let u = it.next().unwrap() as usize;
        let v = it.next().unwrap() as usize;
        let w = it.next().unwrap();
        adj[u].push((v, w));
    }

    let mut dist = vec![INF; n + 1];
    dist[1] = 0;
    let mut heap = BinaryHeap::from([(0i64, 1usize)]);

    while let Some((d, u)) = heap.pop() {
        if d < dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd > dist[v] {
                dist[v] = nd;
                heap.push((nd, v));
            }
        }
    }

    if dist[n] == INF {
        println!("-1");
    } else {
        println!("{}", dist[n]);
    }
}
