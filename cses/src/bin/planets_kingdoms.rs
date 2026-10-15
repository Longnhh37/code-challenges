use std::io::{self, Read};

fn kosaraju(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    fn dfs1(adj: &[Vec<usize>], u: usize, visited: &mut [bool], order: &mut Vec<usize>) {
        visited[u] = true;

        for &v in &adj[u] {
            if !visited[v] {
                dfs1(adj, v, visited, order);
            }
        }

        order.push(u);
    }

    fn dfs2(radj: &[Vec<usize>], u: usize, visited: &mut [bool], scc: &mut Vec<usize>) {
        visited[u] = true;
        scc.push(u);

        for &v in &radj[u] {
            if !visited[v] {
                dfs2(radj, v, visited, scc);
            }
        }
    }

    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);

    for s in 0..n {
        if !visited[s] {
            dfs1(adj, s, &mut visited, &mut order);
        }
    }

    let mut radj = vec![Vec::new(); n];
    for (u, vs) in adj.iter().enumerate() {
        for &v in vs {
            radj[v].push(u);
        }
    }

    visited.fill(false);
    let mut sccs = Vec::new();

    for &s in order.iter().rev() {
        if !visited[s] {
            let mut scc = Vec::new();
            dfs2(&radj, s, &mut visited, &mut scc);
            sccs.push(scc);
        }
    }

    sccs
}

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

    let sccs = kosaraju(&adj);
    let mut res = vec![0; n];

    for (i, scc) in sccs.iter().enumerate() {
        for &v in scc {
            res[v] = i + 1;
        }
    }

    println!("{}", sccs.len());
    let out = res
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{out}");
}
