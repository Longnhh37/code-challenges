impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();

        let mut intervals= intervals.into_iter();

        let mut out = Vec::new();
        let mut cur = intervals.next().unwrap();

        for interval in intervals {
            if cur[1] >= interval[0] {
                cur[1] = cur[1].max(interval[1]);
            } else {
                out.push(cur);
                cur = interval
            }
        }

        out.push(cur);
        out
    }
}
