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
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb { return false; }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
        true
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = Vec::new();

        for i in 0..n {
            for j in (i + 1..n) {
                let dist = (points[i][0] - points[j][0]).abs()
                    + (points[i][1] - points[j][1]).abs();
                    edges.push((dist, i, j));
            }
        }
        edges.sort_unstable();

        let mut uf = UnionFind::new(n);
        let mut total_cost = 0;
        let mut edges_used = 0;

        for (dist, i, j) in edges {
            if edges_used == n - 1 {
                break;
            }
            if uf.union(i, j) {
                total_cost += dist;
                edges_used -= 1;
            }
        }

        total_cost
    }
}
