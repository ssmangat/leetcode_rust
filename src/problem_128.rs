//problem -> 128 [Longest Consecutive Sequence]
//To find the longest consecutive sequence
//1. we will create a hashset and add all the values from the given vector into it, this will
//   remove duplicates
//2. we will create a mutable counter called longest
//3. we will loop through the hashset and use an if condition to find the smallest number in the
//   sequence
//4. once we have the smallest number for a certain sequence, we will run a while loop to find the
//   count of that particular sequence
//5. if the counter is higher than longest then we update longest to higher count.
//
use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn find_longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut longest: i32 = 0;
        for &num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let mut curr_val = num;
                let mut counter = 1;
                while num_set.contains(&(curr_val + 1)) {
                    curr_val += 1;
                    counter += 1;
                }
                longest = longest.max(counter);
            }
        }
        longest
    }
}
