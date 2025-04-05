pub struct Solution {}

impl Solution {
    pub fn product_of_list(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut left = 1;
        for i in 0..nums.len() {
            result[i] *= left;
            left *= nums[i];
        }
        let mut right = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= right;
            right *= nums[i];
        }
        result
    }
}
