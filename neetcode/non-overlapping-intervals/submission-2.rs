impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|k| (k[0], k[1]));
        let old_len = intervals.len();

        let mut it = intervals.into_iter();
        let mut cur = it.next().unwrap();
        let mut res = Vec::new();

        for next in it {
            if cur[1] > next[1] {
                cur = next;
                continue;
            }
            if cur[1] > next[0] {
                continue;
            }
            res.push(std::mem::replace(&mut cur, next));
        }
        res.push(cur);
        (old_len - res.len()) as i32
    }
}
