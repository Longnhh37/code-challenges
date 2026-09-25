impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let k = k as usize;
        let mut stack = Vec::new();

        for b in s.bytes() {
            stack.push(b);
            while stack.len() >= k && stack[stack.len() - k..stack.len() - 1]
                .iter().all(|&b| b == stack[stack.len() - 1]) {
                    for _ in 0..k {
                        stack.pop();
                    }
                }
        }

        String::from_utf8(stack).unwrap()
    }
}
