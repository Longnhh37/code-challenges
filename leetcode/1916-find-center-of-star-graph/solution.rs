impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut counter = vec![0; 10_0001];
        for e in edges {
            counter[e[0] as usize] += 1;
            counter[e[1] as usize] += 1;
        }
        counter.iter().enumerate().map(|(i, c)| (c, i)).max().unwrap().1 as i32
    }
}
