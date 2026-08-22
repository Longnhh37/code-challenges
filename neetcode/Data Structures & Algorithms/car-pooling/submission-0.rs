impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut stops = Vec::new();

        for trip in trips {
            let (passengers, from, to) = (trip[0], trip[1], trip[2]);
            stops.push((from, passengers));
            stops.push((to, -passengers));
        }

        stops.sort_unstable();

        let mut cur = 0;
        for (_, passengers) in stops {
            cur += passengers;
            if cur > capacity {
                return false;
            }
        }
        true
    }
}
