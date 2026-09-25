impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = [None; 128];
        let mut seen = [None; 128];

        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            let sb = s[i] as usize;
            let tb = t[i] as usize;

            if map[sb].is_some() && map[sb] != Some(tb) {
                return false;
            }
            map[sb] = Some(tb);

            if seen[tb].is_some() && seen[tb] != Some(sb) {
                return false;
            }
            seen[tb] = Some(sb);
        }

        true
    }
}
