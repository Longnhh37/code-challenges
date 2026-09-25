impl Solution {
    pub fn insert(
        intervals: Vec<Vec<i32>>, 
        mut new_interval: Vec<i32>
    ) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(intervals.len() + 1);
        let mut it = intervals.into_iter().enumerate();

        while let Some((i, inter)) = it.next() {
            if new_interval[1] < inter[0] {
                res.push(new_interval);
                res.push(inter);
                res.extend(it.map(|(_, v)| v));
                return res;
            } else if new_interval[0] > inter[1] {
                res.push(inter);
            } else {
                new_interval[0] = new_interval[0].min(inter[0]);
                new_interval[1] = new_interval[1].max(inter[1]);
            }
        }

        res.push(new_interval);
        res
    }
}
