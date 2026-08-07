struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len() + 1];
        for (i, n) in nums.into_iter().enumerate() {
            prefix[i + 1] = prefix[i] + n;
        }
        Self { prefix }

    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[right as usize + 1] - self.prefix[left as usize]
    }
}
