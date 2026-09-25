impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut prev = vec![1; n];
        for _ in 0..m - 1 {
            let mut cur = vec![1; n];
            for i in 1..n {
                cur[i] = prev[i] + cur[i - 1];
            }
            prev = cur;
        }

        prev[n - 1 ]
    }
}
