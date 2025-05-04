//problem 247 -> Top K Frequent Elements
//This problem requires use of a HashMap
//1. We iterate over the Vector nums and continue incrementing 1 to numbers already present as keys
//   in HashMap. if they are not in HashMap they get added
//2. We initializze a mutable vector that stores tuple values from our hashmap (key,value)
//3. We sort our array using the values, moving higher values up and lower values down.
//4. we return the top k keys.
//
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        let mut sorted_counter: Vec<(i32, i32)> = counter.into_iter().collect();
        sorted_counter.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_counter
            .iter()
            .take(k as usize)
            .map(|&(num, _)| num)
            .collect()
    }
}
