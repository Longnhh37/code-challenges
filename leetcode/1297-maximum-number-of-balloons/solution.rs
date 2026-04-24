impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut count = [0i32; 26];

        for b in text.bytes() {
            count[(b - b'a') as usize] += 1;
        }

        let l = (b'l' - b'a') as usize;
        let o = (b'o' - b'a') as usize;
        count[l] /= 2;
        count[o] /= 2;

        count[0]
            .min(count[1])
            .min(count[l])
            .min(count[o])
            .min(count[(b'n'- b'a') as usize])
    }
}
