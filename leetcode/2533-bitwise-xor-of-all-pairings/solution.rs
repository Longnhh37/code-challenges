impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len() as i32 & 1;
        let n2 = nums2.len() as i32 & 1;

        let mut res = 0;
        for n in nums1 {
            res ^= (n * n2);
        }
        for n in nums2 {
            res ^= (n * n1);
        }

        res
    }
}
