impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let mut n = nums[i] as usize;
            while i != n - 1 {
                if n as i32 == nums[n - 1] {
                    break;
                }
                nums.swap(i, n - 1);
                n = nums[i] as usize;
            }
        }

        let mut res = Vec::new();
        for (i, &n) in nums.iter().enumerate() {
            let i = i as i32;
            if i != n - 1 {
                res.push(i + 1);
            }
        }

        res
    }
}
