impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();

        for &n in &nums {
            match tails.binary_search(&n) {
                Ok(_) => {}
                Err(idx) => {
                    if idx == tails.len() {
                        tails.push(n);
                    } else {
                        tails[idx] = n;
                    }
                }
            }
        }

        tails.len() as i32
    }

}
