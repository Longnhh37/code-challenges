struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    group: i32,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            group: n as i32,
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
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
        self.group -= 1;
    }
}
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);

        for r in 0..n {
            for c in 0..n {
                if is_connected[r][c] == 1 {
                    uf.union(r, c);
                }
            }
        }

        uf.group
    }
}
