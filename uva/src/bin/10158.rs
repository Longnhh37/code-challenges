use std::fmt::Write;
use std::io::{self, Read};

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<u32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

struct Society {
    n: usize,
    uf: UnionFind,
}

impl Society {
    fn new(n: usize) -> Self {
        Self {
            n,
            uf: UnionFind::new(2 * n),
        }
    }

    fn are_friends(&mut self, a: usize, b: usize) -> bool {
        self.uf.same(a, b)
    }

    fn are_enemies(&mut self, a: usize, b: usize) -> bool {
        self.uf.same(a, b + self.n)
    }

    fn set_friends(&mut self, a: usize, b: usize) -> bool {
        if self.are_enemies(a, b) {
            return false;
        }
        self.uf.union(a, b);
        self.uf.union(a + self.n, b + self.n);
        true
    }

    fn set_enemies(&mut self, a: usize, b: usize) -> bool {
        if self.are_friends(a, b) {
            return false;
        }
        self.uf.union(a, b + self.n);
        self.uf.union(a + self.n, b);
        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|t| t.parse::<usize>().unwrap());
    let n = it.next().unwrap();

    let mut soc = Society::new(n);
    let mut out = String::new();

    while let (Some(c), Some(a), Some(b)) = (it.next(), it.next(), it.next()) {
        match c {
            0 => break,
            1 if !soc.set_friends(a, b) => out.push_str("-1\n"),
            2 if !soc.set_enemies(a, b) => out.push_str("-1\n"),
            1 | 2 => {}
            3 => writeln!(out, "{}", soc.are_friends(a, b) as u8).unwrap(),
            4 => writeln!(out, "{}", soc.are_enemies(a, b) as u8).unwrap(),
            _ => unreachable!("invalid input: {c}"),
        }
    }

    print!("{out}");
}
