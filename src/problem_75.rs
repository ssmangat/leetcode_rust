pub struct Solution {}

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as usize - 1;
        let mut c_nums = nums.clone();
        c_nums.sort();
        let mut count = 0;
        while left < right {
            if c_nums[left] + c_nums[right] == k {
                count += 1;
                left += 1;
                right -= 1;
            } else if c_nums[left] + c_nums[right] < k {
                left += 1;
            } else {
                right -= 1;
            }
        }
        count
    }
}
