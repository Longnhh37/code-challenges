impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let b = s.as_bytes();
        let n = b.len();
        if b[n - 1] == b'1' {
            return false;
        }

        let (min_jump, max_jump) = (min_jump as usize, max_jump as usize);
        let mut i = 0;
        let mut jumped = true;

        while jumped {
            jumped = false;
            let mut l = i + min_jump;
            let mut r = i + max_jump;
            for j in (l..=r).rev() {
                if j >= n {
                    continue;
                } else if j == n - 1 {
                    return true;
                }

                if b[j] == b'0' {
                    i = j;
                    jumped = true;
                }
            }
        }

        false
    }
}
