impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = [0u32; 20_001];
        for n in nums {
            let i = (n + 10_000) as usize;
            counter[i] += 1;
        }
        let mut counter: Vec<(u32, i32)> = counter
            .into_iter()
            .enumerate()
            .map(|(i, c)| (c, i as i32 - 10_000))
            .collect();

        counter.sort_unstable_by(|a, b| b.cmp(&a));

        let mut res = Vec::new();
        for i in 0..k as usize {
            res.push(counter[i].1);
        }
        res
    }
}
