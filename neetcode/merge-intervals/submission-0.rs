impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|k| (k[0], k[1]));
        let mut it = intervals.into_iter();
        let mut cur = it.next().unwrap();
        let mut res = Vec::new();

        while let Some(next) = it.next() {
            if cur[1] >= next[1] {
                continue;
            }
            if cur[1] >= next[0] {
                cur[1] = next[1];
                continue;
            }
            res.push(cur.clone());
            cur = next;
        }

        res.push(cur);
        res
    }
}
