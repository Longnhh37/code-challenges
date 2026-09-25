impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 1 {
            return 0;
        }

        let mut in_mst = vec![false; n];
        let mut min_dist = vec![i32::MAX; n];
        min_dist[0] = 0;

        let dist = |a: usize, b: usize| -> i32 {
            (points[a][0] - points[b][0]).abs() + (points[a][1] - points[b][1]).abs()
        };

        let mut total_cost = 0;
        for _ in 0..n {
            let mut u = usize::MAX;
            let mut best = i32::MAX;
            for v in 0..n {
                if !in_mst[v] && min_dist[v] < best {
                    best = min_dist[v];
                    u = v;
                }
            }
            in_mst[u] = true;
            total_cost += best;

            for v in 0..n {
                if !in_mst[v] {
                    let d = dist(u, v);
                    min_dist[v] = min_dist[v].min(d);
                }
            }
        }

        total_cost
    }
}
