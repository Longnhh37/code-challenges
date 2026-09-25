impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let len = temperatures.len();
        let mut res = vec![0; len];
        let mut stack: Vec<(i32, usize)> = Vec::with_capacity(len);

        for i in (0..len).rev() {
            while let Some(last) = stack.last() && last.0 <= temperatures[i] {
                stack.pop();
            }

            res[i] = if !stack.is_empty() {
                (stack.last().unwrap().1 - i) as i32
            } else {
                0
            };
            stack.push((temperatures[i], i));
        }

        res
    }
}
