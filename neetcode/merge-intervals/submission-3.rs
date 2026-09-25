impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|k| (k[0], k[1]));

        let mut it = intervals.into_iter();
        let mut cur = it.next().unwrap();
        let mut res = Vec::with_capacity(it.size_hint().0);

        for next in it {
            if cur[1] >= next[0] {
                cur[1] = cur[1].max(next[1]);
            } else {
                res.push(std::mem::replace(&mut cur, next));
            }
        }

        res.push(cur);
        res
    }
}
