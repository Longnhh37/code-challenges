impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut res = vec![0; n];
        if k == 0 {
            return res;
        }
        for i in 0..n {
            let mut sum = 0;
            let mut start = if k > 0 { 1 } else { -1 };
            let mut end = k;
            if k < 0 {
                (start, end) = (end, start);
            }
            for j in start..=end {
                let idx = (i as i32 + j + n as i32) % n as i32;
                sum += code[idx as usize];
            }
            res[i] = sum;
        }

        res
    }
}
