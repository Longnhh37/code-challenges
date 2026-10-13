use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let arr: Vec<_> = it.take(n).collect();

    let mut adj = vec![0; n + 1];
    let mut cycle = vec![0; n + 1];

    for (i, &n) in arr.iter().enumerate() {
        adj[i + 1] = n;
    }

    for start in 1..=n {
        let mut jumps = 1;
        let mut visited = vec![false; n + 1];
        visited[start] = true;
        let mut cur = adj[start];

        while !visited[cur] {
            if cycle[cur] != 0 {
                cycle[start] = cycle[cur] + jumps;
                break;
            }
            cur = adj[cur];
            jumps += 1;
        }
        cycle[start] = jumps;
    }

    let mut out = String::with_capacity(n * 2 + 1);
    for i in 1..=n {
        if i > 1 {
            out.push(' ');
        }
        let _ = write!(out, "{}", cycle[i]);
    }
    println!("{out}");
}
