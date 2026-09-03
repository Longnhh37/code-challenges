impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        for (i, w) in haystack.windows(needle.len()).enumerate() {
            if needle == w {
                return i as i32;
            }
        }
        
        -1
    }
}