use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

fn intern<'a>(ids: &mut HashMap<&'a str, usize>, name: &'a str) -> usize {
    let next = ids.len();
    *ids.entry(name).or_insert(next)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut ids: HashMap<&str, usize> = HashMap::new();
    let english = intern(&mut ids, "English");

    let mut targets = Vec::with_capacity(n);
    for _ in 0..n {
        targets.push(intern(&mut ids, it.next().unwrap()));
    }

    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let a = intern(&mut ids, it.next().unwrap());
        let b = intern(&mut ids, it.next().unwrap());
        let w: u64 = it.next().unwrap().parse().unwrap();
        edges.push((a, b, w));
    }

    let len = ids.len();
    let mut adj = vec![Vec::new(); len];
    for &(a, b, _) in &edges {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut hops = vec![usize::MAX; len];
    let mut q = VecDeque::from([english]);
    hops[english] = 0;
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if hops[v] == usize::MAX {
                hops[v] = hops[u] + 1;
                q.push_back(v);
            }
        }
    }

    let inf = u64::MAX;
    let mut best = vec![inf; len];
    best[english] = 0;

    for &(a, b, w) in &edges {
        if hops[a] != usize::MAX && hops[b] != usize::MAX {
            if hops[a] + 1 == hops[b] {
                best[b] = best[b].min(w);
            }
            if hops[b] + 1 == hops[a] {
                best[a] = best[a].min(w);
            }
        }
    }

    let mut total = 0u64;
    for t in targets {
        if hops[t] == usize::MAX {
            println!("Impossible");
            return;
        }
        total += best[t];
    }

    println!("{total}");
}
