impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut len = digits.len() as u32 - 1;
        let mut num = 0;
        for d in digits {
            let d = d as i64;
            match d {
                0 => {},
                _ => num += d * 10_i64.pow(len),
            }
            len -= 1;
        }
        num += 1;

        let mut ans = Vec::new();
        while num > 0 {
            ans.push((num % 10) as i32);
            num /= 10;
        }

        ans.reverse();
        ans
    }
}
