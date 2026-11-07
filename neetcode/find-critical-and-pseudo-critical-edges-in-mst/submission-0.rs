struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

impl Solution {
    fn kruskal(
        n: usize,
        edges: &[(usize, usize, i32, usize)], // (u, v, w, original index)
        skip: Option<usize>,
        force: Option<usize>,
    ) -> (i32, bool) {
        let mut dsu = Dsu::new(n);
        let mut weight = 0;
        let mut used = 0;

        if let Some(f) = force {
            let (u, v, w, _) = edges[f];
            if dsu.union(u, v) {
                weight += w;
                used += 1;
            }
        }
        for (i, &(u, v, w, _)) in edges.iter().enumerate() {
            if Some(i) == skip {
                continue;
            }
            if dsu.union(u, v) {
                weight += w;
                used += 1;
            }
        }
        (weight, used == n - 1)
    }

    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut sorted: Vec<(usize, usize, i32, usize)> = edges
            .iter()
            .enumerate()
            .map(|(i, e)| (e[0] as usize, e[1] as usize, e[2], i))
            .collect();
        sorted.sort_unstable_by_key(|e| e.2);

        let (base, _) = Self::kruskal(n, &sorted, None, None);

        let (mut critical, mut pseudo) = (Vec::new(), Vec::new());
        for i in 0..sorted.len() {
            let orig = sorted[i].3 as i32;

            // critical: remove it disconnect the graph or increase MST weight
            let (w_skip, connected) = Self::kruskal(n, &sorted, Some(i), None);
            if !connected || w_skip > base {
                critical.push(orig);
                continue;
            }
            // pseudo critical: force it in still achieve the optimal weight
            let (w_force, _) = Self::kruskal(n, &sorted, None, Some(i));
            if w_force == base {
                pseudo.push(orig);
            }
        }

        vec![critical, pseudo]
    }
}
