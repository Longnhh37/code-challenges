use std::collections::VecDeque;
use std::fmt::Write as _;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
    cap: i32,
}

struct Dinic {
    graph: Vec<Vec<usize>>,
    edges: Vec<Edge>,
    level: Vec<i32>,
    ptr: Vec<usize>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            edges: Vec::new(),
            level: vec![-1; n],
            ptr: vec![0; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i32) -> usize {
        let id = self.edges.len();
        self.edges.push(Edge { to: v, cap });
        self.edges.push(Edge { to: u, cap: 0 });
        self.graph[u].push(id);
        self.graph[v].push(id + 1);
        id
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level.fill(-1);
        self.level[s] = 0;
        let mut q = VecDeque::from([s]);

        while let Some(u) = q.pop_front() {
            for &id in &self.graph[u] {
                let Edge { to, cap } = self.edges[id];
                if cap > 0 && self.level[to] == -1 {
                    self.level[to] = self.level[u] + 1;
                    q.push_back(to);
                }
            }
        }

        self.level[t] >= 0
    }

    fn dfs(&mut self, u: usize, t: usize, f: i32) -> i32 {
        if u == t {
            return f;
        }

        while self.ptr[u] < self.graph[u].len() {
            let id = self.graph[u][self.ptr[u]];
            let Edge { to, cap } = self.edges[id];

            if cap > 0 && self.level[to] == self.level[u] + 1 {
                let pushed = self.dfs(to, t, f.min(cap));
                if pushed > 0 {
                    self.edges[id].cap -= pushed;
                    self.edges[id ^ 1].cap += pushed;
                    return pushed;
                }
            }

            self.ptr[u] += 1;
        }

        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i32 {
        let mut flow = 0;
        while self.bfs(s, t) {
            self.ptr.fill(0);
            loop {
                let pushed = self.dfs(s, t, i32::MAX);
                if pushed == 0 {
                    break;
                }
                flow += pushed;
            }
        }
        flow
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let (n, m, k) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());

    let src = 0;
    let sink = n + m + 1;
    let mut dinic = Dinic::new(n + m + 2);

    for boy in 1..=n {
        dinic.add_edge(src, boy, 1);
    }
    for girl in 1..=m {
        dinic.add_edge(n + girl, sink, 1);
    }

    let mut edges = Vec::with_capacity(k);
    for _ in 0..k {
        let (a, b) = (it.next().unwrap(), it.next().unwrap());
        let id = dinic.add_edge(a, n + b, 1);
        edges.push((a, b, id));
    }

    let flow = dinic.max_flow(src, sink);

    let mut out = String::new();
    writeln!(out, "{flow}").unwrap();
    for (a, b, id) in edges {
        if dinic.edges[id].cap == 0 {
            writeln!(out, "{a} {b}").unwrap();
        }
    }
    println!("{out}");
}
