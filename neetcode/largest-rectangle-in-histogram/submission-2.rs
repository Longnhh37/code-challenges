impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut res = 0;

        for (i, &h) in heights.iter().enumerate() {
            let i = i as i32;
            let mut start = i;
            while let Some(&(_, last_h)) = stack.last() && last_h > h {
                let (popped_i, popped_h) = stack.pop().unwrap();
                res = res.max((i - popped_i) * popped_h);
                start = popped_i;
            }
            stack.push((start, h));
        }

        let n = heights.len() as i32;
        for &(i, h) in stack.iter() {
            res = res.max((n - i) * h);
        }

        res
    }
}
