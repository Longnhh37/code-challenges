impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j) = (m - 1 , n - 1);
        let mut last = (m + n) as usize - 1;

        while j >= 0 {
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                nums1[last] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[last] = nums2[j as usize];
                j -= 1;
            }
            last = last.saturating_sub(1);
        }
    }
}
