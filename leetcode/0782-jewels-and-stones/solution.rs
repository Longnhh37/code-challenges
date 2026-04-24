impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut seen = [false; 52];

        for b in jewels.bytes() {
            seen[idx(b)] = true;
        }

        stones
            .bytes()
            .filter(|&b| seen[idx(b)])
            .count() as i32
    }
}

fn idx(b: u8) -> usize {
    if b.is_ascii_lowercase() {
        (b - b'a') as usize
    } else {
        (b - b'A' + 26) as usize
    }
}
