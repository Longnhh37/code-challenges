impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();

        if b2.len() < b1.len() {
            return false;
        }

        let mut c1 = [0; 26];
        let mut c2 = [0; 26];

        for &b in b1 {
            c1[(b - b'a') as usize] += 1;
        }

        let win = b1.len();

        for i in 0..win {
            c2[(b2[i] - b'a') as usize] += 1;
        }

        let mut matches = (0..26).filter(|&i| c1[i] == c2[i]).count();
        if matches == 26 {
            return true;
        }

        for r in win..b2.len() {
            let l = r - win;

            let add = (b2[r] - b'a') as usize;
            if c1[add] == c2[add] {
                matches -= 1;
            }
            c2[add] += 1;
            if c1[add] == c2[add] {
                matches += 1;
            }

            let rem = (b2[l] - b'a') as usize;
            if c1[rem] == c2[rem] {
                matches -= 1;
            }
            c2[rem] -= 1;
            if c1[rem] == c2[rem] {
                matches += 1;
            }

            if matches == 26 {
                return true;
            }
        }

        false
    }
}
