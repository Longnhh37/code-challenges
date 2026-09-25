impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;
        let mut counter = vec![0usize; 1001];
        for h in hand {
            counter[h as usize] += 1;
        }

        for i in 0..counter.len() {
            let cur = counter[i];
            if cur == 0 {
                continue;
            }
            for j in i..i + group_size {
                if counter[j] < cur {
                    return false;
                }
                counter[j] -= cur;
            }
        }

        true
    }
}
