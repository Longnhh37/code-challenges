impl Solution {
    pub fn integer_break(num: i32) -> i32 {
        if num <= 3 {
            return num - 1;
        }

        match num % 3 {
            0 => 3_i32.pow((num / 3) as u32),
            1 => 3_i32.pow((num / 3 - 1) as u32) * 4,
            _ => 3_i32.pow((num / 3) as u32) * 2,
        }
    }
}
