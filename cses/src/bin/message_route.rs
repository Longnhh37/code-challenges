use std::collections::VecDeque;
use std::fmt::Write;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut adj = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let u = it.next().unwrap();
        let v = it.next().unwrap();
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut parent: Vec<Option<usize>> = vec![None; n + 1];
    let mut q = VecDeque::from([1]);

    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            match parent[v] {
                Some(_) => continue,
                None => {
                    parent[v] = Some(u);
                    q.push_back(v);
                }
            }
        }
    }

    let mut path = vec![n];

    match parent[n] {
        None => {
            println!("IMPOSSIBLE");
            return;
        }
        Some(mut pos) => {
            while pos != 1 {
                path.push(pos);
                pos = parent[pos].unwrap();
            }
        }
    }

    path.push(1);
    let mut out = String::with_capacity(path.len() * 7);
    for (i, &p) in path.iter().rev().enumerate() {
        if i > 0 {
            out.push(' ');
        }

        let _ = write!(out, "{}", p);
    }

    println!("{}", path.len());
    println!("{}", out);
}
