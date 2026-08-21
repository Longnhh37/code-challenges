impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let side = sum / 4;
        
        let mut sticks = matchsticks;
        sticks.sort_unstable_by(|a, b| b.cmp(a));

        if sticks[0] > side {
            return false;
        }

        let mut sides = [0; 4];
        Self::backtrack(&sticks, 0, &mut sides, side)
    }

    fn backtrack(sticks: &[i32], idx: usize, sides: &mut [i32; 4], target: i32) -> bool {
        if idx == sticks.len() {
            return sides.iter().all(|&s| s == target);
        }

        let stick = sticks[idx];
        for i in 0..4 {
            if sides[i] + stick > target {
                continue;
            }

            if i > 0 && sides[i] == sides[i - 1] {
                continue;
            }

            sides[i] += stick;
            if Self::backtrack(sticks, idx + 1, sides, target) {
                return true;
            }
            sides[i] -= stick;
        }

        false
    }


}
