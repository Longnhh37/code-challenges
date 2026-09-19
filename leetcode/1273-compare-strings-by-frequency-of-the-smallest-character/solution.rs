impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let f = |s: &str| -> u16 {
            s
                .bytes()
                .map(|b| (b - b'a') as usize)
                .fold([0u16; 26], |mut acc, x| {
                    acc[x] += 1;
                    acc
                })
                .iter()
                .filter(|&&c| c > 0)
                .nth(0).copied().unwrap()
        };

        let queries: Vec<u16> = queries
            .iter()
            .map(|s| f(s))
            .collect();
        
        let mut words: Vec<u16> = words
            .iter()
            .map(|s| f(s))
            .collect();

        words.sort_unstable();
        let n = words.len();

        let mut res = Vec::new();
        for q in queries {
            let v = n - words.partition_point(|&w| w <= q);
            res.push(v as i32);
        }

        res
    }
    
}
