use std::collections::VecDeque;
use std::io::Read;

const NEG_INF: i64 = i64::MIN;

fn bfs(start: usize, adj: &[Vec<usize>]) -> Vec<bool> {
    let mut seen = vec![false; adj.len()];
    seen[start] = true;
    let mut q = VecDeque::from([start]);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if !seen[v] {
                seen[v] = true;
                q.push_back(v);
            }
        }
    }
    seen
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());
    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;

    let mut edges = Vec::with_capacity(m);
    let mut adj = vec![Vec::new(); n + 1];
    let mut radj = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let u = it.next().unwrap() as usize;
        let v = it.next().unwrap() as usize;
        let w = it.next().unwrap();
        edges.push((u, v, w));
        adj[u].push(v);
        radj[v].push(u);
    }

    let from_start = bfs(1, &adj);
    let to_end = bfs(n, &radj);
    let useful = |x: usize| from_start[x] && to_end[x];
    edges.retain(|&(u, v, _)| useful(u) && useful(v));

    let mut dist = vec![NEG_INF; n + 1];
    dist[1] = 0;

    for round in 0..n {
        let mut changed = false;
        for &(u, v, w) in &edges {
            if dist[u] != NEG_INF && dist[u] + w > dist[v] {
                dist[v] = dist[u] + w;
                changed = true;
            }
        }
        if !changed {
            break;
        }
        if round == n - 1 {
            println!("-1");
            return;
        }
    }

    println!("{}", dist[n]);
}
