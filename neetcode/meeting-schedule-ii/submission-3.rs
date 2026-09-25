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
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let n = intervals.len();

        let (mut starts, mut ends) = (Vec::new(), Vec::new());
        for int in intervals {
            starts.push(int.start);
            ends.push(int.end);
        }

        starts.sort_unstable();
        ends.sort_unstable();
        let (mut most, mut cur) = (0i32, 0i32);
        let (mut s, mut e) = (0usize, 0usize);

        while s < n {
            if starts[s] < ends[e] {
                s += 1;
                cur += 1;
            } else {
                e += 1;
                cur -= 1;
            }
            most = most.max(cur);
        }

        most
    }
}
