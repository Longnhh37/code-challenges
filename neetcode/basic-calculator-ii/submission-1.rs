impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut chars = s.bytes().filter(|&b| b != b' ').peekable();
        let mut stack = Vec::new();
        let mut sign = b'+';

        while let Some(&b) = chars.peek() {
            if b.is_ascii_digit() {
                let num = Self::read_number(&mut chars);
                match sign {
                    b'+' => stack.push(num),
                    b'-' => stack.push(-num),
                    b'*' => {
                        let top = stack.last_mut().unwrap();
                        *top *= num;
                    }
                    b'/' => {
                        let top = stack.last_mut().unwrap();
                        *top /= num;
                    }
                    _ => unreachable!(),
                }
            } else {
                sign = b;
                chars.next();
            }
        }

        stack.iter().sum()
    }

    fn read_number(chars: &mut std::iter::Peekable<impl Iterator<Item = u8>>) -> i32 {
        let mut num = 0i32;
        while let Some(&b) = chars.peek() {
            if !b.is_ascii_digit() {
                break;
            }
            num = num * 10 + (b - b'0') as i32;
            chars.next();
        }
        num
    }
}
