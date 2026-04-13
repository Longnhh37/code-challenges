use std::collections::HashSet;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut count = [0; 26];

        for b in magazine.bytes() {
            count[(b - b'a') as usize] += 1;
        }

        for b in ransom_note.bytes() {
            let idx = (b - b'a') as usize;
            match count[idx] {
                0 => return false,
                _ => count[idx] -= 1, 
            }
        }

        true   
    }
}
