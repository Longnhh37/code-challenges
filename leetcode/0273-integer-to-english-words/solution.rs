const ONES: [&str; 10] = [
    "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

const TEENS: [&str; 10] = [
    "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
];

const TENS: [&str; 10] = [
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

const UNIT_SUFFIX: [&str; 4] = [
    "", "Thousand", "Million", "Billion",
];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let mut n = num as u32;
        if n == 0 {
            return "Zero".to_string();
        }
        let mut chunks = Vec::new();
        let mut unit = 0;

        while n > 0 {
            let chunk = n % 1000;
            if chunk != 0 {
                chunks.push((chunk, unit));
            }
            n /= 1000;
            unit += 1;
        }

        let mut res = String::new();

        for (i, &(chunk, unit)) in chunks.iter().rev().enumerate() {
            if i > 0 {
                res.push(' ');
            }
            Self::write_under_1000(chunk, &mut res);
            if !UNIT_SUFFIX[unit].is_empty() {
                res.push(' ');
                res.push_str(UNIT_SUFFIX[unit]);
            }
        }

        res
    }

    fn write_under_1000(mut n: u32, s: &mut String) {
        if n >= 100 {
            s.push_str(ONES[(n / 100) as usize]);
            s.push_str(" Hundred");

            if n.is_multiple_of(100) {
                return;
            }
            s.push(' ');
            n %= 100;
        }
        if n < 10 {
            s.push_str(ONES[n as usize]);
            return;
        }
        if n < 20 {
            s.push_str(TEENS[(n - 10) as usize]);
            return;
        }
        let o = n % 10;
        let t = n / 10;
        s.push_str(TENS[t as usize]);
        if o != 0 {
            s.push(' ');
            s.push_str(ONES[o as usize]);
        }
    }
}
