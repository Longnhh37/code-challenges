use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut st = HashMap::new();
        let mut ts = HashMap::new();

        for i in 0..s.len() {
            let cur_s = s[i];
            let cur_t = t[i];

            let contains_s = st.contains_key(&cur_s);
            let contains_t = ts.contains_key(&cur_t);

            if !contains_s && !contains_t {
                st.insert(cur_s, cur_t);
                ts.insert(cur_t, cur_s);
            } else if contains_s && contains_t {
                if st.get(&cur_s) !=  Some(&cur_t) || ts.get(&cur_t) != Some(&cur_s) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
