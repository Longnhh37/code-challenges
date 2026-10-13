use std::io::{self, Read};

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
    largest: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
            components: n,
            largest: 0,
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
        self.components -= 1;
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.size[ra] += self.size[rb];
        self.parent[rb] = ra;
        self.largest = self.largest.max(self.size[ra]);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let mut uf = UnionFind::new(n);

    for _ in 0..it.next().unwrap() {
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        uf.union(a, b);
        println!("{} {}", uf.components, uf.largest);
    }
}
