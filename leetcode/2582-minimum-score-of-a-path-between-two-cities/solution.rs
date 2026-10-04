struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            rank: vec![0; n + 1],
            weight: vec![i32::MAX; n + 1],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize,  b: usize, w: i32) {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra != rb {
            if self.rank[ra] < self.rank[rb] {
                std::mem::swap(&mut ra, &mut rb);
            }
            self.parent[rb] = ra;
            if self.rank[ra] == self.rank[rb] {
                self.rank[ra] += 1;
            }
            self.weight[ra] = self.weight[ra].min(self.weight[rb]);
        }
        self.weight[ra] = self.weight[ra].min(w);
    }
}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for r in &roads {
            let (u, v, w) = (r[0] as usize, r[1] as usize, r[2]);
            uf.union(u, v, w);
        }
        let rn = uf.find(n);
        uf.weight[rn]
    }
}
