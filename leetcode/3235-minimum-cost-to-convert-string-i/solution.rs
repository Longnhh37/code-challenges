const INF: u64 = u64::MAX / 2;

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let mut dist = [[INF; 26]; 26];
        for i in 0..26 {
            dist[i][i] = 0;
        }

        for i in 0..original.len() {
            let u = (original[i] as u8 - b'a') as usize;
            let v = (changed[i] as u8 - b'a') as usize;
            let w = cost[i] as u64;
            dist[u][v] = dist[u][v].min(w);
        }

        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    let via_k = dist[i][k] + dist[k][j];
                    if via_k < dist[i][j] {
                        dist[i][j] = via_k;
                    }
                }
            }
        }

        let mut total = 0;
        for (ub, vb) in source.bytes().zip(target.bytes()) {
            let u = (ub - b'a') as usize;
            let v = (vb - b'a') as usize;
            if dist[u][v] == INF {
                return -1;
            }
            total += dist[u][v];
        }

        total as i64
    }
}
