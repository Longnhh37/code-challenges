impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut res = Vec::new();

        for ope in &operations {
            if let Ok(n) = ope.parse::<i32>() {
                res.push(n);
                continue;
            }

            if ope == "+" {
                if let [.., a, b] = res.as_slice() {
                    res.push(a + b);
                }
            } else if ope == "C" {
                res.pop();
            } else {
                let n = res.last().unwrap();
                res.push(n * 2);
            }
        }

        res.iter().sum::<i32>()
    }
}

