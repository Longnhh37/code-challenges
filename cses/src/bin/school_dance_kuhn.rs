use std::fmt::Write as _;
use std::io::{self, Read};

fn kuhn(u: usize, adj: &[Vec<usize>], visited: &mut [bool], matched: &mut [Option<usize>]) -> bool {
    for &v in &adj[u] {
        if visited[v] {
            continue;
        }
        visited[v] = true;

        if matched[v].is_none_or(|w| kuhn(w, adj, visited, matched)) {
            matched[v] = Some(u);
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let (m, n, k) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
    let mut adj = vec![Vec::new(); n];

    for _ in 0..k {
        let (u, v) = (it.next().unwrap() - 1, it.next().unwrap() - 1);
        adj[u].push(v);
    }

    let mut matched = vec![None; m];
    let mut visited = vec![false; m];
    let mut count = 0;

    for u in 0..n {
        visited.fill(false);
        if kuhn(u, &adj, &mut visited, &mut matched) {
            count += 1;
        }
    }

    let mut out = String::new();
    writeln!(out, "{count}").unwrap();
    for (girl, boy) in matched.iter().enumerate() {
        if let Some(boy) = boy {
            writeln!(out, "{} {}", boy + 1, girl + 1).unwrap();
        }
    }
    println!("{out}");
}
