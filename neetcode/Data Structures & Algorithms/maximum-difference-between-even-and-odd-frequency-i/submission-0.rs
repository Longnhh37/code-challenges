impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut cnt = [0; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        let max = cnt.iter().filter(|&c| c % 2 == 1).max().unwrap();
        let min = cnt.iter().filter(|&&c| c != 0 && c % 2 == 0).min().unwrap();

        max - min
    }
}
