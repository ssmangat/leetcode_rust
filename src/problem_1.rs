//problem 1 -> Two Sum
//This problem can be solved in various manners, i am using HashMap.
//1. We will iterate over the nums vector
//2. We will calculate number by subtracting value at index i from target
//3. if number is present in hashmap, we return the index of that value along with current index
//4. if number is not present we add current value to hashmap as key, with it's value being it's index
//5. return empty vector if nothing is found
use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::with_capacity(nums.len());
        for (index, &value) in nums.iter().enumerate() {
            let number = target - value;
            if let Some(&idx) = seen.get(&number) {
                return vec![idx, index as i32];
            }
            seen.insert(value, index as i32);
        }
        vec![]
    }
}
