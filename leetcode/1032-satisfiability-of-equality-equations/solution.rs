struct UnionFind {
    parent: [usize; 26],
    rank: [u8; 26],
}

impl UnionFind {
    fn new() -> Self {
        Self { 
            parent: std::array::from_fn(|i| i),
            rank: [0; 26],
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
        if self.rank[ra] < self.rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        if self.rank[ra] == self.rank[rb] {
            self.rank[ra] += 1;
        }
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let idx = |b: u8| (b - b'a') as usize;
        let mut uf = UnionFind::new();


        for e in equations.iter().map(|s| s.as_bytes()).filter(|e| e[1] == b'=') {
            uf.union(idx(e[0]), idx(e[3]));
        }

        equations
            .iter()
            .map(|s| s.as_bytes())
            .filter(|e| e[1] == b'!')
            .all(|e| uf.find(idx(e[0])) != uf.find(idx(e[3])))
    }
}
