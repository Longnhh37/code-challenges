use std::cmp::Ordering;
use std::io::{Read, Write};

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[x] != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }

        match self.rank[ra].cmp(&self.rank[rb]) {
            Ordering::Less => self.parent[ra] = rb,
            Ordering::Greater => self.parent[rb] = ra,
            Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut edges: Vec<(u64, usize, usize, usize)> = Vec::with_capacity(m);
    for idx in 0..m {
        let a = it.next().unwrap() - 1;
        let b = it.next().unwrap() - 1;
        let w = it.next().unwrap() as u64;
        edges.push((w, a, b, idx));
    }

    edges.sort();

    let mut uf = UnionFind::new(n);
    let mut res = vec![""; m];

    let mut i = 0;
    while i < edges.len() {
        let mut j = i;
        while j < edges.len() && edges[j].0 == edges[i].0 {
            j += 1;
        }
        for k in i..j {
            let (_, a, b, orig_idx) = edges[k];
            res[orig_idx] = if uf.find(a) != uf.find(b) {
                "YES"
            } else {
                "NO"
            };
        }
        for k in i..j {
            let (_, a, b, _) = edges[k];
            uf.union(a, b);
        }
        i = j;
    }

    let mut out = String::with_capacity(res.len() * 4);
    for s in res {
        out.push_str(s);
        out.push('\n');
    }

    std::io::stdout().write_all(out.as_bytes()).unwrap();
}
