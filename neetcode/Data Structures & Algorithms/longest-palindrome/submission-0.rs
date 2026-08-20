impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counter = [0; 128];
        for b in s.bytes() {
            counter[b as usize] += 1;
        }

        let mut longest = 0;
        let mut is_odd = 0;

        for i in b'A'..=b'Z' {
            let cnt = counter[i as usize];
            longest += (cnt / 2) * 2;
            is_odd |= cnt & 1;
        }

        for i in b'a'..=b'z' {
            let cnt = counter[i as usize];
            longest += (cnt / 2) * 2;
            is_odd |= cnt & 1;
        }
        
        longest + is_odd
    }
}