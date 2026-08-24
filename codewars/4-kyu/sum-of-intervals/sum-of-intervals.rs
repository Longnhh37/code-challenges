fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut intervals: Vec<(i64, i64)> = intervals
        .iter()
        .map(|&(a, b)| (a as i64, b as i64))
        .collect();
    intervals.sort_unstable();
    
    let mut res = Vec::new();
    let mut cur = intervals[0];
    
    for r in 1..intervals.len() {
        if cur.1 >= intervals[r].0 {
            cur.1 = cur.1.max(intervals[r].1);
        } else {
            res.push(cur);
            cur = intervals[r];
        }
    }
    
    res.push(cur);
    res.iter().map(|&(a, b)| b - a).sum::<i64>() as i32
}
​