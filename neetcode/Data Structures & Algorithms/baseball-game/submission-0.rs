impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        let mut last = 0;

        for op in operations {
            match op.as_str() {
                "+" => {
                    stack.push(stack[last - 1] + stack[last - 2]);
                    last += 1;
                }
                "C" => {
                    stack.pop();
                    last -= 1;
                } 
                "D" => {
                    stack.push(stack[last - 1] * 2);
                    last += 1;
                }
                _ => {
                    stack.push(op.parse::<i32>().unwrap());
                    last += 1;
                }
            }
        }

        stack.into_iter().sum::<i32>()
    }
}
