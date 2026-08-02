impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|k| (k[0], k[1]));

        let mut it = intervals.into_iter();
        let mut cur_end = it.next().unwrap()[1];
        let mut removed = 0;

        for next in it {
            if cur_end > next[0] {
                removed += 1;
                cur_end = cur_end.min(next[1]);
            } else {
                cur_end = next[1];
            }
        }
        removed
    }
}
