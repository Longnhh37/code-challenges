impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        if n == 1 {
            return 1;
        }

        let mut in_deg = vec![0; n + 1];
        let mut trust_other = vec![false; n + 1];

        for t in trust {
            let (u, v) = (t[0] as usize, t[1] as usize);
            in_deg[v] += 1;
            trust_other[u] = true;
        }

        for i in 0..=n {
            if in_deg[i] == n - 1 && !trust_other[i] {
                return i as i32;
            }
        }
        -1
    }
}
