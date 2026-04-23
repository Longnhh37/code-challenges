impl Solution {
  fn match_roman_char(c: char) -> i32 {
        match c {
            'I'=>1,
            'V'=>5,
            'X'=>10,
            'L'=>50,
            'C'=>100,
            'D'=>500,
            'M'=>1000,
            _=>0
        }
   }

   fn roman_to_int(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    let mut total = 0;

    for (i, &c) in chars.iter().enumerate() {
        let v = Self::match_roman_char(c);

        if i + 1 < len && v < Self::match_roman_char(chars[i+1]) {
            total -= v;
        } else {
            total += v;
        }
    }

    total
   }
}
