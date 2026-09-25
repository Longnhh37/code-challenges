impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = [130; 128];
        let mut seen = [130; 128];

        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            let sb = s[i] as usize;
            let tb = t[i] as usize;

            if map[sb] != 130 && map[sb] != tb {
                return false;
            }
            map[sb] = tb;

            if seen[tb] != 130 && seen[tb] != sb {
                return false;
            }
            seen[tb] = sb;
        }

        true
    }
}
