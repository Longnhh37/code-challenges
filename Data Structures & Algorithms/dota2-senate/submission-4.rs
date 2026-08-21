impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut s: Vec<u8> = senate.as_bytes().to_vec();
        let mut cnt = 0i32;
        let mut i = 0;

        while i < s.len() {
            let c = s[i];
            if c == b'R' {
                if cnt < 0 {
                    s.push(b'D');
                }
                cnt += 1;
            } else {
                if cnt > 0 {
                    s.push(b'R');
                }
                cnt -= 1;
            }
            i += 1;
        }

        if cnt > 0 {
            "Radiant".to_string()
        } else {
            "Dire".to_string()
        }
    }
}