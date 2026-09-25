impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trustor = vec![false; n + 1];
        let mut trustee = vec![0_usize; n + 1];

        for item in &trust {
            let (a, b) = (item[0], item[1]);
            trustor[a as usize] = true;
            trustee[b as usize] += 1;
        }

        for i in 1..n + 1 {
            if trustor[i] {
                continue;
            }
            if trustee[i] == n - 1 {
                return i as i32;
            }
        }
        -1
    }
}
