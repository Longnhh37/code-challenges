// The guess API is already defined for you.
// fn guess(num: i64) -> i32:
//     -1 if num is higher than the picked number
//      1 if num is lower than the picked number
//      0 if num is equal to the picked number

impl Solution {
    pub unsafe fn guess_number(n: i64) -> i64 {
        let (mut l, mut r) = (1, n);

        loop {
            let mid = l + ((r - l) >> 1);
            let res = unsafe { guess(mid) };
            if res > 0 {
                l = mid + 1;
            } else if res < 0 {
                r = mid - 1;
            } else {
                return mid;
            }
        }
    }
}
