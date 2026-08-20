impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut outgoing = vec![0_usize; n + 1];
        let mut incoming = vec![0_usize; n + 1];

        for t in &trust {
            outgoing[t[0] as usize] = 1;
            incoming[t[1] as usize] += 1;
        }

        for i in 1..n + 1 {
            if outgoing[i] == 0 && incoming[i] == n - 1 {
                return i as i32;
            }
        }

        -1
    }
}
