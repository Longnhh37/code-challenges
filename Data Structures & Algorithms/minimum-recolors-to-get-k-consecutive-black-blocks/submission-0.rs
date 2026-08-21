impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let b = blocks.as_bytes();
        let mut res = i32::MAX;

        for w in b.windows(k) {
            let c = w.iter().filter(|&&b| b == b'W').count() as i32;
            res = res.min(c);
        }
        
        res
    }
}
