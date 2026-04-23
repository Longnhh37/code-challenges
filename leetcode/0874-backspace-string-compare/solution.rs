impl Solution {
    fn next_valid(s: &[u8], mut i: Option<usize>)
        -> (Option<usize>, Option<u8>)
    {
        let mut skip = 0;

        while let Some(p) = i {
            if s[p] == b'#' {
                skip += 1;
            } else if skip > 0 {
                skip -= 1;
            } else {
                return (p.checked_sub(1), Some(s[p]));
            }

            i = p.checked_sub(1);
        }

        (None, None)
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut i = s.len().checked_sub(1);
        let mut j = t.len().checked_sub(1);

        loop {
            let (ni, a) = Self::next_valid(s, i);
            let (nj, b) = Self::next_valid(t, j);

            if a != b {
                return false;
            }

            if a.is_none() {
                return true;
            }

            i = ni;
            j = nj;
        }
    }
}
