pub struct Solution {}

impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
        let mut l = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(l, i);
                l += 1
            }
        }
    }
}
