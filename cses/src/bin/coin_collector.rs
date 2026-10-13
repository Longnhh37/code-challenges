use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let coins: Vec<usize> = it.by_ref().take(n).collect();
    let mut adj = vec![Vec::new(); n];
    let mut indeg = vec![0; n];

    for _ in 0..m {
        let u = it.next().unwrap() - 1;
        let v = it.next().unwrap() - 1;
        adj[u].push(v);
        indeg[v] += 1;
    }
}
