impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        for b in bookings {
            let u = b[0] as usize - 1;
            let v = b[1] as usize - 1;
            let seats = b[2];
            for i in u..=v {
                res[i] += seats;
            }
        }
        res
    }
}
