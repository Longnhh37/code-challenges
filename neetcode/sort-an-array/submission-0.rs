impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&mut nums);       
        nums
    }

    fn merge_sort(arr: &mut [i32]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        {
            let (left, right) = arr.split_at_mut(mid);
            Self::merge_sort(left);
            Self::merge_sort(right);
        }

        Self::merge(arr, mid);
    }

    fn merge(arr: &mut [i32], mid: usize) {
        let mut merged: Vec<i32> = Vec::with_capacity(arr.len());
        let (left, right) = arr.split_at(mid);
        let (mut i, mut j) = (0, 0);

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                merged.push(left[i]);
                i += 1;
            } else {
                merged.push(right[j]);
                j += 1;
            }
        }

        merged.extend_from_slice(&left[i..]);
        merged.extend_from_slice(&right[j..]);

        arr.copy_from_slice(&merged);
    }
}
