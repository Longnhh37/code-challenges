use std::io::{self, Read};

fn first_unreachable(adj: &[Vec<usize>]) -> Option<usize> {
    let mut seen = vec![false; adj.len()];
    let mut stack = vec![0];
    seen[0] = true;

    while let Some(u) = stack.pop() {
        for &v in &adj[u] {
            if !seen[v] {
                seen[v] = true;
                stack.push(v);
            }
        }
    }

    seen.iter().position(|&s| !s)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut fwd = vec![Vec::new(); n];
    let mut rev = vec![Vec::new(); n];

    for _ in 0..m {
        let u = it.next().unwrap() - 1;
        let v = it.next().unwrap() - 1;
        fwd[u].push(v);
        rev[v].push(u);
    }

    if let Some(v) = first_unreachable(&fwd) {
        println!("NO\n1 {}", v + 1);
    } else if let Some(v) = first_unreachable(&rev) {
        println!("NO\n{} 1", v + 1);
    } else {
        println!("YES");
    }
}
