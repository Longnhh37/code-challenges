impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        for cur in asteroids {
            Self::collision_process(&mut stack, cur);
        }

        stack
    }

    fn collision_process(stack: &mut Vec<i32>, cur: i32) {
        if stack.is_empty() {
            stack.push(cur);
            return;
        }
        let last = *stack.last().unwrap();
        if cur > 0 && last > 0
            || cur < 0 && last < 0
            || last < 0 && cur > 0 {
            stack.push(cur);
            return;
        } 
        if cur == -last {
            stack.pop();
            return;
        } 
        if cur + last > 0 {
            return;
        }
        // cur + last < 0
        stack.pop();
        Self::collision_process(stack, cur);
    }
}
