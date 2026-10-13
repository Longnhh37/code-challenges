use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

struct UnionFind {
    parent: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        self.components -= 1;
        self.parent[rb] = ra;
        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let mut heap = BinaryHeap::new();

    for _ in 0..it.next().unwrap() {
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        let w = it.next().unwrap();
        heap.push(Reverse((w, a, b)));
    }

    let mut total = 0;
    let mut uf = UnionFind::new(n);

    while let Some(Reverse((w, a, b))) = heap.pop() {
        if uf.union(a, b) {
            total += w;
        }
    }

    if uf.components == 1 {
        println!("{total}");
    } else {
        println!("IMPOSSIBLE")
    }
}
