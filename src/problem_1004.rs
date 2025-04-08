pub struct Solution {}

impl Solution {
    pub fn max_consecutive_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut k = k;
        for right in &nums {
            if *right == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[left] == 0 {
                    k += 1;
                }
                left += 1;
            }
        }
        nums.len() as i32 - left as i32
    }
}
