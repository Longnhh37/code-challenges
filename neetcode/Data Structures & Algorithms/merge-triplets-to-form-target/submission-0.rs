impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut cur = vec![0, 0, 0];
        for t in &triplets {
            if t[0] > target[0] || t[1] > target[1] || t[2] > target[2] {
                continue;
            }
            cur[0] = cur[0].max(t[0]);
            cur[1] = cur[1].max(t[1]);
            cur[2] = cur[2].max(t[2]);
        }

        cur == target
    }
}
