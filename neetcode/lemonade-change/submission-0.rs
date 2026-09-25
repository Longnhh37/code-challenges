impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = [0; 2];

        for bill in bills {
            match bill {
                5 => change[0] += 1,
                10 => {
                    if change[0] == 0 {
                        return false;
                    }
                    change[0] -= 1;
                    change[1] += 1;
                }
                20 => {
                    if change[1] > 0 && change[0] > 0 {
                        change[0] -= 1;
                        change[1] -= 1;
                        continue;
                    }
                    if change[0] >= 3 {
                        change[0] -= 3;
                        continue;
                    }
                    return false;
                }
                _ => unreachable!(),
            }
        }

        true
    }
}
