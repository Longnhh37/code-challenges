impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let ni = n as i32;
        
        for i in 0..n {
            let mut cur = nums[i];
            while 1 <= cur && cur <= ni && nums[i] != nums[cur as usize - 1] {
                nums.swap(i, cur as usize - 1);
                cur = nums[i];
            }
        }

        for (i, &n) in nums.iter().enumerate() {
            let i = i as i32;
            if n != i + 1 {
                return i + 1;
            }
        }

        ni + 1
    }
}