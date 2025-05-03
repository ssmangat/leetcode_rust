//Contains Duplicate - Problem 217
//Logic is simple, we use a hashset to save the values of the array provided, since hashsets can
//only have a single value once, if the value already exists, we return true else we insert the
//value, if the loop ends without finding a value that's duplicate, return False.

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(nums.len()); //hashset with capacity of nums vector.
        for num in nums.into_iter() {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        false
    }
}
