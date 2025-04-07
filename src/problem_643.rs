pub struct Solution {}

impl Solution {
    pub fn maximum_average_subarray(nums: Vec<i32>, k: i32) -> f64 {
        let k: usize = k as usize;
        let mut curr: i32 = nums[..k].iter().sum::<i32>();
        let mut maxx: i32 = curr;
        for i in k..nums.len() {
            curr += nums[i] - nums[i - k];
            maxx = maxx.max(curr);
        }
        (maxx as f64 / k as f64 * 100000.0).round() / 100000.0
    }
}
