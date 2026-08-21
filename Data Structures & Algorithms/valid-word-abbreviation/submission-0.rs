impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let word = word.as_bytes();
        let abbr = abbr.as_bytes();
        let (m, n) = (word.len(), abbr.len());
        let (mut i, mut j) = (0, 0);

        while i < m && j < n {
            if abbr[j] == b'0' {
                return false;
            }

            if abbr[j].is_ascii_alphabetic() {
                if word[i] == abbr[j] {
                    i += 1;
                    j += 1;
                    continue;
                } else {
                    return false;
                }
            }

            let mut sub_len = 0usize;
            while j < n && abbr[j].is_ascii_digit() {
                sub_len = sub_len * 10 + (abbr[j] - b'0') as usize;
                j += 1;
            }
            i += sub_len;
        }

        i == m && j == n
    }
}
