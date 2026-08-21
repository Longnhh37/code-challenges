impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut s = String::new();

        let mut i = 0;
        while i < n {
            s.push(chars[i]);
            let mut j = i + 1;
            while j < n && chars[i] == chars[j] {
                j += 1;
            }

            if j - i > 1 {
                s.push_str(&(j - i).to_string());
            }
            i = j;
        }
        for (idx, c) in s.chars().enumerate() {
            chars[idx] = c;
        }

        s.len() as i32
    }
}
