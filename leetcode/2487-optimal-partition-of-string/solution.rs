impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut res = 0;
        let mut seen = 0u32;

        for b in s.bytes() {
            let bit = 1u32 << (b - b'a');
            if seen & bit != 0 {
                seen = 0;
                res += 1;
            } 
            seen |= bit;
        }
        
        res + 1
    }
}
