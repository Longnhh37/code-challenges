impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut stack: Vec<i64> = Vec::new();
        let mut last_op = b'+';
        let mut i = 0;

        while i < s.len() {
            match s[i] {
                b' ' => i += 1,
                b'0'..=b'9' => {
                    let mut cur: i64 = 0;
                    while i < s.len() && s[i].is_ascii_digit() {
                        cur = cur * 10 + (s[i] - b'0') as i64;
                        i += 1;
                    }
                    match last_op {
                        b'+' => stack.push(cur),
                        b'-' => stack.push(-cur),
                        b'*' => {
                            let lhs = stack.pop().unwrap();
                            stack.push(lhs * cur);
                        }
                        b'/' => {
                            let lhs = stack.pop().unwrap();
                            stack.push(lhs / cur);
                        }
                        _ => unreachable!(),
                    }
                }
                op @ (b'+' | b'-' | b'*' | b'/') => {
                    last_op = op;
                    i += 1;
                }
                _ => unreachable!(),
            }
        }

        stack.iter().sum::<i64>() as i32
    }
}
