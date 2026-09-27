impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let mut no_del = arr[0];
        let mut with_del = i32::MIN;
        let mut best = arr[0];

        for &n in &arr[1..] {
            with_del = with_del.saturating_add(n).max(no_del);
            no_del = n.max(no_del + n);
            best = best.max(no_del).max(with_del);
        }

        best
    }
}
