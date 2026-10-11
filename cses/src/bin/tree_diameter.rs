use std::io::{self, Read};

fn farthest(adj: &[Vec<usize>], start: usize) -> (usize, usize) {
    let mut stack = vec![(start, 0, 0)];
    let mut farthest = (start, 0);

    while let Some((u, parent, dist)) = stack.pop() {
        if dist > farthest.1 {
            farthest = (u, dist);
        }
        for &v in &adj[u] {
            if v != parent {
                stack.push((v, u, dist + 1));
            }
        }
    }

    farthest
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();

    let mut adj = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let u = it.next().unwrap();
        let v = it.next().unwrap();
        adj[u].push(v);
        adj[v].push(u);
    }

    let (a, _) = farthest(&adj, 1);
    let (_, diameter) = farthest(&adj, a);

    println!("{diameter}");
}
