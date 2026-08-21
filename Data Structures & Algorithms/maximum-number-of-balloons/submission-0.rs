impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt = [0; 26];
        for b in text.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        i32::MAX
            .min(cnt[0])
            .min(cnt[1])
            .min(cnt[11] / 2)
            .min(cnt[13])
            .min(cnt[14] / 2)
    }
}
