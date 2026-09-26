use std::collections::VecDeque;
use std::io::Read;

const INF: u128 = u128::MAX / 2;

struct Dinic {
    to: Vec<usize>,
    cap: Vec<u128>,
    graph: Vec<Vec<usize>>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        Self {
            to: Vec::new(),
            cap: Vec::new(),
            graph: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, c: u128) {
        self.graph[u].push(self.to.len());
        self.to.push(v);
        self.cap.push(c);
        self.graph[v].push(self.to.len());
        self.to.push(u);
        self.cap.push(0);
    }

    fn bfs(&self, s: usize, t: usize, level: &mut [i32]) -> bool {
        level.iter_mut().for_each(|x| *x -= 1);
        level[s] = 0;
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &id in &self.graph[u] {
                let v = self.to[id];
                if self.cap[id] > 0 && level[v] < 0 {
                    level[v] = level[u] + 1;
                    q.push_back(v);
                }
            }
        }
        level[t] >= 0
    }

    fn dfs(&mut self, u: usize, t: usize, f: u128, level: &[i32], it: &mut [usize]) -> u128 {
        if u == t {
            return f;
        }
        while it[u] < self.graph[u].len() {
            let id = self.graph[u][it[u]];
            let v = self.to[id];
            if self.cap[id] > 0 && level[v] == level[u] + 1 {
                let d = self.dfs(v, t, f.min(self.cap[id]), level, it);
                if d > 0 {
                    self.cap[id] -= d;
                    self.cap[id ^ 1] += d;
                    return d;
                }
            }
            it[u] += 1;
        }
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> u128 {
        let n = self.graph.len();
        let mut flow = 0;
        let mut level = vec![-1; n];
        while self.bfs(s, t, &mut level) {
            let mut it = vec![0usize; n];
            loop {
                let f = self.dfs(s, t, INF, &level, &mut it);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u128>().unwrap());

    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap();

    let mut dinic = Dinic::new(n + 1);
    for _ in 0..m {
        let u = it.next().unwrap() as usize;
        let v = it.next().unwrap() as usize;
        let c = it.next().unwrap();
        dinic.add_edge(u, v, c);
    }

    println!("{}", dinic.max_flow(1, n));
}
