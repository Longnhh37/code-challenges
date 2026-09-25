impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut j = 0;
        for i in m as usize..nums1.len() {
            nums1[i] = nums2[j]; 
            j += 1;
        }

        nums1.sort_unstable();
    }
}
