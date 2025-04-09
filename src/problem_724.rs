pub struct Solution {}

impl Solution {
    pub fn find_pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_max = 0;
        let total: i32 = nums.iter().sum();
        for i in 0..nums.len() {
            let right_max = total - left_max - nums[i];
            if right_max == left_max {
                return i as i32;
            }
            left_max += nums[i];
        }
        return -1i32;
    }
}
