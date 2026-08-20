/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_unstable_by_key(|int| int.start);
        for w in intervals.windows(2) {
            let prev = &w[0];
            let next = &w[1];
            if prev.end > next.start {
                return false;
            }
        }

        true
    }
}
