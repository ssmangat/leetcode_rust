pub struct Solution {}

impl Solution {
    pub fn longest_subarray_after_removing_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut k = 1;
        let mut maxx = 0;
        for right in 0..nums.len() {
            if nums[right] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[left] == 0 {
                    k += 1
                }
                left += 1
            }
            maxx = maxx.max(right - left)
        }
        maxx as i32
    }
}
