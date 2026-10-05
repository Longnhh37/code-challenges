use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut uf = UnionFind::new(n);

    for _ in 0..m {
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        uf.union(a, b);
    }

    for i in 1..=n {
        uf.find(i);
    }

    let uniq: Vec<_> = uf
        .parent
        .iter()
        .skip(1)
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    println!("{}", uniq.len() - 1);
    let a = uniq[0];
    for b in uniq.into_iter().skip(1) {
        println!("{} {}", a, b);
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            rank: vec![0; n + 1],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match ra.cmp(&rb) {
            Ordering::Less => self.parent[ra] = rb,
            Ordering::Greater => self.parent[rb] = ra,
            Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }
}
