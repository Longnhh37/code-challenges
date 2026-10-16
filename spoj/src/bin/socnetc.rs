use std::fmt::Write;
use std::io::{self, Read};

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<u64>,
    limit: u64,
}

impl UnionFind {
    fn new(n: usize, limit: u64) -> Self {
        Self {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
            limit,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        let total = self.size[ra] + self.size[rb];
        if total > self.limit {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.size[ra] = total;
        self.parent[rb] = ra;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: u64 = it.next().unwrap().parse().unwrap();
    let query: u64 = it.next().unwrap().parse().unwrap();

    let mut uf = UnionFind::new(n, m);
    let mut out = String::new();

    for _ in 0..query {
        match it.next().unwrap() {
            "S" => {
                let x: usize = it.next().unwrap().parse().unwrap();
                let rx = uf.find(x);
                let _ = writeln!(out, "{}", uf.size[rx]);
            }
            "A" => {
                let a: usize = it.next().unwrap().parse().unwrap();
                let b: usize = it.next().unwrap().parse().unwrap();
                uf.union(a, b);
            }
            "E" => {
                let a: usize = it.next().unwrap().parse().unwrap();
                let b: usize = it.next().unwrap().parse().unwrap();
                out.push_str(if uf.find(a) == uf.find(b) {
                    "YES\n"
                } else {
                    "NO\n"
                });
            }
            _ => unreachable!(),
        }
    }

    println!("{out}");
}
