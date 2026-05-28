impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        for num in nums.iter().rev() {
            let mut num = *num;

            while num > 0 {
                res.push(num % 10);
                num /= 10;
            }
        }

        res.into_iter().rev().collect::<Vec<i32>>()
    }
}
