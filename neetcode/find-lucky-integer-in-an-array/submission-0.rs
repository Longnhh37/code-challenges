impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut cnt = [0usize; 501];
        for n in arr {
            cnt[n as usize] += 1;
        }
        for (i, c) in cnt.into_iter().enumerate().rev() {
            if i == c && i != 0 {
                return c as i32;
            }
        }
        -1
    }
}
