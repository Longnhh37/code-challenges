use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
        let n = special_roads.len();
        let tgt = (target[0], target[1]);
        let manhattan = |a: (i32, i32), b: (i32, i32)| (a.0 - b.0).abs() + (a.1 - b.1).abs();

        let mut pos = Vec::with_capacity(n + 1);
        pos.push((start[0], start[1]));
        pos.extend(special_roads.iter().map(|r| (r[2], r[3])));

        let mut dist = vec![i32::MAX; n + 1];
        let mut heap = BinaryHeap::new();

        dist[0] = 0;
        heap.push(Reverse((0, 0)));

        let mut ans = manhattan(pos[0], tgt);

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            ans = ans.min(d + manhattan(pos[u], tgt));

            for (i, r) in special_roads.iter().enumerate() {
                let nd = d + manhattan(pos[u], (r[0], r[1])) + r[4];
                if nd < dist[i + 1] {
                    dist[i + 1] = nd;
                    heap.push(Reverse((nd, i + 1)));
                }
            }
        }

        ans
    }
}
