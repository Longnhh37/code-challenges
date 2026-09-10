impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut cnt = [0; 2];

        for (i, b) in num.bytes().enumerate() {
            cnt[i & 1] += (b - b'0') as u16;
        }

        cnt[0] == cnt[1]
    }
}
