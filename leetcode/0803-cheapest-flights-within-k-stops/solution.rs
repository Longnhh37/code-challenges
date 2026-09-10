impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst, k) = (n as usize, src as usize, dst as usize, k as usize);
        let mut dist = vec![i32::MAX; n];
        dist[src] = 0;

        for _ in 0..=k {
            let snapshot = dist.clone();

            for f in &flights {
                let (u, v, w) = (f[0] as usize, f[1] as usize, f[2]);
                if snapshot[u] != i32::MAX && snapshot[u] + w < dist[v] {
                    dist[v] = snapshot[u] + w;
                }
            }
        }

        if dist[dst] == i32::MAX { -1 } else { dist[dst] }
    }
}
