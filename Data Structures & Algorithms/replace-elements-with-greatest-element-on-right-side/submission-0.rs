impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        for v in arr.iter_mut().rev() {
            if *v > max {
                (*v, max) = (max, *v);
            } else {
                *v = max;
            }
        }
        arr
    }
}
