impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        const INF: i32 = i32::MAX;
        let (n, src, dst) = (n as usize, src as usize, dst as usize);

        let mut dist = vec![INF; n];
        dist[src] = 0;

        for _ in 0..=k {
            let prev_dist = dist.clone();
            let mut updated = false;

            for flight in &flights {
                let (u, v, price) = (flight[0], flight[1], flight[2]);
                let (u, v) = (u as usize, v as usize);
                if prev_dist[u] == INF {
                    continue;
                }
                let new_price = prev_dist[u] + price;
                if new_price < dist[v] {
                    dist[v] = new_price;
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }

        if dist[dst] == INF { - 1 } else { dist[dst] }
    }
}
