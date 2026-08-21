impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        let mut total = 0;

        for (i, &c) in chars.iter().enumerate() {
            let n = Self::get_roman_char(c);

            if i + 1 < len && n < Self::get_roman_char(chars[i + 1]) {
                total -= n;
            } else {
                total += n;
            }
        }
        
        total
    }

    fn get_roman_char(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}
