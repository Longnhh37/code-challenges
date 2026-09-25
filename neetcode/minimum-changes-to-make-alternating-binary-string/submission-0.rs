impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut cnt = 0;

        for (i, &c) in bytes.iter().enumerate() {
            if i % 2 == 0 {
                if c == b'0' { cnt += 1; }
            } else {
                if c == b'1' { cnt += 1; }
            }
        }

        (cnt.min(bytes.len() - cnt)) as i32
        
    }
}