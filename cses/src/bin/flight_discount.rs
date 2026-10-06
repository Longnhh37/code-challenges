use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

type Graph = Vec<Vec<(usize, u64)>>;

const INF: u64 = u64::MAX / 4;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());

    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;

    let mut adj: Graph = vec![Vec::new(); n + 1];
    let mut radj: Graph = vec![Vec::new(); n + 1];
    let mut edges = Vec::with_capacity(m);

    for _ in 0..m {
        let u = it.next().unwrap() as usize;
        let v = it.next().unwrap() as usize;
        let w = it.next().unwrap();
        adj[u].push((v, w));
        radj[v].push((u, w));
        edges.push((u, v, w));
    }

    let d1 = dijkstra(&adj, 1);
    let dn = dijkstra(&radj, n);

    let res = edges
        .iter()
        .filter(|&&(u, v, _)| d1[u] < INF && dn[v] < INF)
        .map(|&(u, v, w)| d1[u] + w / 2 + dn[v])
        .min()
        .unwrap();

    println!("{}", res);
}

fn dijkstra(adj: &Graph, src: usize) -> Vec<u64> {
    let mut dist = vec![INF; adj.len()];
    let mut heap = BinaryHeap::from([Reverse((0, src))]);
    dist[src] = 0;

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

    dist
}
