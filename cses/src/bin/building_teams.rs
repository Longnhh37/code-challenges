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
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut color = vec![0u8; n + 1];
    let mut q = VecDeque::new();

    for s in 1..=n {
        if color[s] != 0 {
            continue;
        }
        color[s] = 1;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if color[v] == 0 {
                    color[v] = 3 - color[u];
                    q.push_back(v);
                } else if color[v] == color[u] {
                    println!("IMPOSSIBLE");
                    return;
                }
            }
        }
    }

    let mut out = String::with_capacity(n * 3);
    for (i, &c) in color[1..].iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        if c == 1 {
            let _ = write!(out, "1");
        } else {
            let _ = write!(out, "2");
        }
    }

    println!("{}", out);
}
