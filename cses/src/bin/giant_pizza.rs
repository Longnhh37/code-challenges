use std::io::{self, Read};

struct TwoSat {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl TwoSat {
    fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); 2 * n],
        }
    }

    fn node(var: usize, value: bool) -> usize {
        var * 2 + usize::from(!value)
    }

    fn add_clause(&mut self, a: usize, va: bool, b: usize, vb: bool) {
        let (na, nb) = (Self::node(a, va), Self::node(b, vb));
        self.adj[na ^ 1].push(nb);
        self.adj[nb ^ 1].push(na);
    }

    fn solve(&self) -> Option<Vec<bool>> {
        let sccs = kosaraju(&self.adj);

        let mut comp = vec![0; 2 * self.n];
        for (idx, scc) in sccs.iter().enumerate() {
            for &v in scc {
                comp[v] = idx;
            }
        }

        let mut res = vec![false; self.n];
        for i in 0..self.n {
            let (t, f) = (2 * i, 2 * i + 1);
            if comp[t] == comp[f] {
                return None;
            }
            res[i] = comp[t] > comp[f];
        }

        Some(res)
    }
}

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

    let mut it = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut twosat = TwoSat::new(m);

    for _ in 0..n {
        let va = it.next().unwrap() == "+";
        let a = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let vb = it.next().unwrap() == "+";
        let b = it.next().unwrap().parse::<usize>().unwrap() - 1;
        twosat.add_clause(a, va, b, vb);
    }

    match twosat.solve() {
        None => println!("IMPOSSIBLE"),
        Some(res) => {
            let out = res
                .into_iter()
                .map(|b| if b { "+" } else { "-" })
                .collect::<Vec<_>>()
                .join(" ");
            println!("{out}");
        }
    }
}
