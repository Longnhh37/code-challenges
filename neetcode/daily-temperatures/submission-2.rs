impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let len = temperatures.len();
        let mut res = vec![0; len];
        let mut stack: Vec<usize> = Vec::with_capacity(len);

        for i in 0..len {
            while let Some(&last) = stack.last() && temperatures[last] < temperatures[i] {
                stack.pop();
                res[last] = (i - last) as i32;
            }
            stack.push(i);
        }

        res
    }
}
