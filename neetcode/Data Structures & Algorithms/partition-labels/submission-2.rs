impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut bytes = s.as_bytes();
        let n = bytes.len();

        let mut last = [0_usize; 26];
        for (i, &b) in bytes.iter().enumerate() {
            last[(b - b'a') as usize] = i;
        }

        let mut res = Vec::new();
        let mut start = 0usize;
        let mut end = 0usize;

        for (i, &b) in bytes.iter().enumerate() {
            end = end.max(last[(b - b'a') as usize]);
            if i == end {
                res.push((end - start + 1) as i32);
                start = i + 1;
            }
        }

        res
    }
}
