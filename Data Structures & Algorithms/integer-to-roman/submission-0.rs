impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut res = Vec::new();
        let mut unit = 1;

        while num > 0 {
            let mut d = num % 10;
            while d > 0 {
                match d {
                    1..=3 | 6..=8 => {
                        res.push(Self::get_roman_char(1, unit));
                        d -= 1;
                    }
                    4 => {
                        res.push(Self::get_roman_char(5, unit));
                        res.push(Self::get_roman_char(1, unit));
                        break;
                    }
                    5 => {
                        res.push(Self::get_roman_char(5, unit));
                        break;
                    }
                    9 => {
                        res.push(Self::get_roman_char(1, unit * 10));
                        res.push(Self::get_roman_char(1, unit));
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            num /= 10;
            unit *= 10;
        }

        res.reverse();
        String::from_utf8(res).unwrap()
    }

    fn get_roman_char(d: i32, unit: i32) -> u8 {
        match (d, unit) {
            (1, 1) => b'I',
            (5, 1) => b'V',
            (1, 10) => b'X',
            (5, 10) => b'L',
            (1, 100) => b'C',
            (5, 100) => b'D',
            (1, 1000) => b'M',
            _ => unreachable!(),
        }
    }
}
