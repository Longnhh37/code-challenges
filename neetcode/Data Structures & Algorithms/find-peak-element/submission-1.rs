impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut arr = vec![i32::MIN];
        arr.extend(nums);
        arr.push(i32::MIN);

        for i in 1..arr.len() - 1 {
            if arr[i - 1] < arr[i] && arr[i] > arr[i + 1] {
                return i as i32 - 1;
            }
        }
        0
    }
}
