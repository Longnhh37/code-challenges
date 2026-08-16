impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(u8, i32)> = Vec::new();

        for b in s.bytes() {
            if let Some(last) = stack.last_mut() {
                if last.0 == b {
                    last.1 += 1;
                    if last.1 == k {
                        stack.pop();
                    }
                    continue;
                }
            }
            stack.push((b, 1));
        }

        let mut res = String::with_capacity(s.len());
        for (b, cnt) in stack {
            for _ in 0..cnt {
                res.push(b as char);
            }
        }
        res
    }
}
