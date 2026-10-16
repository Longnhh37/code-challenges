use std::io::{self, Read};

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    cyc: Vec<bool>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            cyc: vec![false; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn add_edge(&mut self, a: usize, b: usize) -> bool {
        let (mut ra, mut rb) = (self.find(a), self.find(b));

        if ra == rb {
            if self.cyc[ra] {
                return false;
            }
            self.cyc[ra] = true;
            return true;
        }

        if self.cyc[ra] && self.cyc[rb] {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.cyc[ra] |= self.cyc[rb];
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
    let l = it.next().unwrap();

    let mut dsu = Dsu::new(l + 1);
    let mut out = String::new();

    for _ in 0..n {
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        out.push_str(if dsu.add_edge(a, b) {
            "LADICA\n"
        } else {
            "SMECE\n"
        });
    }

    print!("{out}");
}
