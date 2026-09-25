impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut k = 0;
        let mut i = 0;

        while i < n {
            chars[k] = chars[i];
            k += 1;
            let mut j = i + 1;
            while j < n && chars[i] == chars[j] {
                j += 1;
            }

            if j - i > 1 {
                for c in (j - i).to_string().chars() {
                    chars[k] = c;
                    k += 1;
                }
            }
            i = j;
        }
        
        k as i32
    }
}
