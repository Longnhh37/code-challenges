use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let root = 1;

    let mut parent = vec![0usize; n + 1];
    let mut pending = vec![0usize; n + 1];

    for v in 2..=n {
        let u = it.next().unwrap();
        parent[v] = u;
        pending[u] += 1;
    }

    let mut stack: Vec<usize> = (1..=n).filter(|&i| pending[i] == 0).collect();
    let mut sub = vec![0usize; n + 1];

    while let Some(u) = stack.pop() {
        if u == root {
            continue;
        }
        let p = parent[u];
        sub[p] += sub[u] + 1;
        pending[p] -= 1;
        if pending[p] == 0 {
            stack.push(p);
        }
    }

    let out = sub[1..]
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(" ");

    println!("{out}");
}
