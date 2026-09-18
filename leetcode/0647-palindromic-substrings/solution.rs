use std::collections::HashSet;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut seen = HashSet::new();

        for i in 0..s.len() {
            Self::expand(&s, i, i, &mut seen);
            Self::expand(&s, i, i + 1, &mut seen);
        }

        seen.len() as i32
    }

    fn expand(
        s: &[u8], 
        start: usize, 
        end: usize, 
        seen: &mut HashSet<(usize, usize)>
    ) {
        let mut l = start as isize;
        let mut r = end;

        while l >= 0 && r < s.len() {
            if s[l as usize] == s[r] {
                seen.insert((l as usize, r));
                l -= 1;
                r += 1;
            } else {
                return;
            }
        }
    } 
}
