impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let n = needle.len();

        for (i, w) in haystack.windows(n).enumerate() {
            if w == needle {
                return i as i32;
            }
        }

        -1
    }
}
