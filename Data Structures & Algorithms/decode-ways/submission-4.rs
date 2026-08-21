impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();

        if bytes.is_empty() || bytes[0] == b'0' {
            return 0;
        }

        let (mut prev2, mut prev) = (1, 1);

        for i in 1..bytes.len() {
            let mut cur = 0;

            if bytes[i] != b'0' {
                cur += prev;
            }

            let num = (bytes[i - 1] - b'0') * 10 + (bytes[i] - b'0');
            if (10..=26).contains(&num) {
                cur += prev2;
            }

            (prev2, prev) = (prev, cur);
        }

        prev
    }
}
