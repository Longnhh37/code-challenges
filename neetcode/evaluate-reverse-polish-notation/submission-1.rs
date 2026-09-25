impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for tok in tokens {
            match tok.parse::<i32>() {
                Ok(n) => stack.push(n),
                Err(_) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    match tok.as_str() {
                        "+" => stack.push(left + right),
                        "-" => stack.push(left - right),
                        "*" => stack.push(left * right),
                        "/" => stack.push(left / right),
                        _ => unreachable!(),
                    }
                }
            }
        }

        stack.pop().unwrap()
    }
}
