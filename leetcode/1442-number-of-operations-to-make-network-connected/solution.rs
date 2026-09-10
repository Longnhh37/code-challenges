struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
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

        if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else {
            self.parent[ra] = rb;
            self.rank[rb] += 1;
        }

        self.count -= 1;
    }
}
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if connections.len() + 1 < n {
            return -1;
        }

        let mut uf = UnionFind::new(n as usize);
        for conn in &connections {
            uf.union(conn[0] as usize, conn[1] as usize);
        }
        (uf.count - 1) as i32
    }
}
