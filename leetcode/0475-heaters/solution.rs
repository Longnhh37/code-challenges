impl Solution {
    pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        heaters.sort_unstable();
        let mut max_radius = 0;

        for house in houses {
            let i = heaters.partition_point(|&h| h < house);
            let mut min_dist = i32::MAX;
            
            if i < heaters.len() {
                min_dist = min_dist.min(heaters[i] - house);
            }
            if i > 0 {
                min_dist = min_dist.min(house - heaters[i - 1]);
            }
            max_radius = max_radius.max(min_dist);
        }

        max_radius
    }
}
