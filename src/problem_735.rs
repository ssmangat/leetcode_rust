pub struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        for asteroid in asteroids {
            let mut collision = true;
            while collision && asteroid < 0 && *stack.last().unwrap() > 0 {
                if -asteroid > *stack.last().unwrap() {
                    stack.pop();
                } else if -asteroid == *stack.last().unwrap() {
                    stack.pop();
                    collision = false;
                } else {
                    collision = false;
                }
            }
            if collision {
                stack.push(asteroid);
            }
        }
        stack
    }
}
