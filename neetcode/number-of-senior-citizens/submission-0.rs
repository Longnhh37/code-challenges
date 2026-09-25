impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut res = 0;
        for d in details {
            let age = d[11..13].parse::<u8>().unwrap();
            if age > 60 {
                res += 1;
            }
        }
        res
    }
}
