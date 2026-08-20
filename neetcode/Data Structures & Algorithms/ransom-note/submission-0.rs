impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0; 26];
        for b in magazine.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        for b in ransom_note.bytes() {
            if cnt[(b - b'a') as usize] == 0 {
                return false;
            }
            cnt[(b - b'a') as usize] -= 1;
        }

        true
    }
}
