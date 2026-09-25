impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let n = people.len();

        let (mut l, mut r) = (0, n - 1);
        let mut count = 0;

        while 0 <= r && people[r] == limit {
            if r == 0 {
                count += 1;
                return count;
            }
            count += 1;
            r -= 1;
        }

        while l < r {
            let sum = people[l] + people[r];

            if sum > limit {
                count += 1;
                r -= 1;
            } else {
                count += 1;
                l += 1;
                r -= 1;
            }
        }
        if l == r { count += 1; }
        count
    }
}
