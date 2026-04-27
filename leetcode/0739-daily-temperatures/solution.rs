impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let n = t.len();
        let mut ans = vec![0; n];

        for i in (0..n-1).rev() {
            let mut j = i + 1;

            while j < n {
                if t[j] > t[i] {
                    ans[i] = (j - i) as i32;
                    break;
                }

                if ans[j] == 0 {
                    break;
                }

                j += ans[j] as usize;
            }
        }

        ans
    }
}

