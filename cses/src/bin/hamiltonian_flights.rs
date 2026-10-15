use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let mut adj = vec![Vec::new(); n];

    for _ in 0..it.next().unwrap() {
        let a = it.next().unwrap() - 1;
        let b = it.next().unwrap() - 1;
        adj[a].push(b);
    }

    let mut cnt = 0;
    let mut visited = vec![false; n];
    backtrack(&adj, 0, &mut visited, 0, &mut cnt);
    println!("{}", cnt);
}

fn backtrack(adj: &[Vec<usize>], u: usize, visited: &mut [bool], path: usize, cnt: &mut u64) {
    if u == adj.len() - 1 {
        if path == adj.len() - 1 {
            *cnt += 1;
            *cnt %= MOD;
        }
        return;
    }

    if visited[u] {
        return;
    }
    visited[u] = true;
    for &v in &adj[u] {
        backtrack(adj, v, visited, path + 1, cnt);
    }
    visited[u] = false;
}
