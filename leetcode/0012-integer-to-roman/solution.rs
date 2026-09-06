impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let int_roman = [(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), 
            (90, "XC"), (50, "L"), (40, "XL"), (10, "X"),
            (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
        ];

        let mut res = String::new();
        let mut i = 0;

        while num > 0 {
            let (div, s) = int_roman[i];
            if num >= div {
                res.push_str(&s.repeat((num / div) as usize));
                num %= div;
            } else {
                i += 1;
            }
        }

        res
    }
}
