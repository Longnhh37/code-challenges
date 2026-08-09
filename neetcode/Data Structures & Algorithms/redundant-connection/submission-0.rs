struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false;
        }

        if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else {
            self.parent[px] = py;
            self.rank[py] += 1;
        }

        true
    }
}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len() + 1);
        for e in &edges {
            let (x, y) = (e[0], e[1]);
            if !uf.union(x as usize, y as usize) {
                return vec![x, y];
            }
        }
        unreachable!();
    }
}
