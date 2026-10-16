use std::collections::VecDeque;
use std::io::{self, Read};

const ROOT: usize = 1;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut next = || it.next().unwrap();

    let n: usize = next().parse().unwrap();

    let mut parent = vec![(0usize, 0.0f64, false); n + 1];
    let mut children = vec![Vec::new(); n + 1];
    for _ in 0..n - 1 {
        let u: usize = next().parse().unwrap();
        let v: usize = next().parse().unwrap();
        let p: f64 = next().parse().unwrap();
        let is_super = next() == "1";
        parent[v] = (u, p, is_super);
        children[u].push(v);
    }

    let mut dist = vec![0.0f64; n + 1];
    for d in dist.iter_mut().skip(1) {
        let k: f64 = next().parse().unwrap();
        *d = k.max(0.0);
    }

    let mut order = Vec::with_capacity(n);
    let mut q = VecDeque::from([ROOT]);
    while let Some(u) = q.pop_front() {
        order.push(u);
        q.extend(&children[u]);
    }

    for &v in order.iter().skip(1).rev() {
        let (u, p, is_super) = parent[v];
        let need = if is_super { dist[v].sqrt() } else { dist[v] };
        dist[u] = dist[u].max(need * 100.0 / p);
    }

    println!("{}", dist[ROOT]);
}
