impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (b1, b2, b3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (n, m) = (b1.len(), b2.len());

        if n + m != b3.len() {
            return false;
        }
        let mut dp: u128 = 0;

        for i in 0..=n {
            for j in 0..=m {
                let ok = if i == 0 && j == 0 {
                    true
                } else {
                    let from_top = i > 0 && (dp >> j) & 1 == 1 && b1[i - 1] == b3[i + j - 1];
                    let from_left = j > 0 && (dp >> (j - 1)) & 1 == 1 && b2[j - 1] == b3[i + j - 1];
                    from_top || from_left
                };

                dp = (dp & !(1u128 << j)) | ((ok as u128) << j);
            }
        }

        (dp >> m) & 1 == 1
    }
}
