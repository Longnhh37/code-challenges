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
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (pa, pb) = (self.find(a), self.find(b));

        if pa == pb {
            return;
        }

        if self.rank[pa] < self.rank[pb] {
            self.parent[pa] = pb;
        } else if self.rank[pa] > self.rank[pb] {
            self.parent[pb] = pa;
        } else {
            self.parent[pb] = pa;
            self.rank[pa] += 1;
        }
    }
}

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for e in edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            uf.union(a, b);
        }
        let mut roots: Vec<usize> = (0..n).map(|i| uf.find(i)).collect();
        roots.sort_unstable();
        roots.dedup();
        roots.len() as i32
    }
}
