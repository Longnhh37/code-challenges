use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut adj = vec![Vec::new(); n + 1];
    let mut in_deg = vec![0i32; n + 1];

    for _ in 0..m {
        let u = it.next().unwrap();
        let v = it.next().unwrap();
        adj[u].push(v);
        in_deg[v] += 1;
    }

    let mut q: VecDeque<_> = in_deg
        .iter()
        .skip(1)
        .enumerate()
        .filter(|(_, c)| **c == 0)
        .map(|(i, _)| i + 1)
        .collect();

    let mut res = Vec::with_capacity(n);

    while let Some(idx) = q.pop_front() {
        res.push(idx);
        for &nei in &adj[idx] {
            in_deg[nei] -= 1;
            if in_deg[nei] == 0 {
                q.push_back(nei);
            }
        }
    }

    if res.len() == n {
        let to_print = res
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{to_print}")
    } else {
        println!("IMPOSSIBLE")
    }
}
