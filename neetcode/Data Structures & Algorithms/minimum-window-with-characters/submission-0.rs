impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() || s.len() < t.len() {
            return String::new();
        }

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut need = [0i32; 128];
        for &b in t_bytes {
            need[b as usize] += 1;
        }

        let required = need.iter().filter(|&&x| x > 0).count();
        let mut formed = 0;

        let mut window_counts = [0i32; 128];

        let mut left = 0;
        let mut best_len = usize::MAX;
        let mut best_left = 0;

        for right in 0..s_bytes.len() {
            let c = s_bytes[right] as usize;
            window_counts[c] += 1;

            if need[c] > 0 && window_counts[c] == need[c] {
                formed += 1;
            }

            while formed == required {
                if right - left + 1 < best_len {
                    best_len = right - left + 1;
                    best_left = left;
                }

                let left_char = s_bytes[left] as usize;
                window_counts[left_char] -= 1;
                if need[left_char] > 0 && window_counts[left_char] < need[left_char] {
                    formed -= 1;
                }
                left += 1;
            }
        }

        if best_len == usize::MAX {
            String::new()
        } else {
            String::from_utf8(s_bytes[best_left..best_left + best_len].to_vec()).unwrap()
        }
    }
}
