use std::collections::{HashMap, HashSet};
pub struct Solution {}

impl Solution {
    pub fn unique_number_of_occurrences(nums: Vec<i32>) -> bool {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut hs: HashSet<i32> = HashSet::new();
        for i in &nums {
            *hm.entry(*i).or_insert(0) += 1;
        }
        let has_duplicates = hm.values().any(|value| hs.insert(*value));
        has_duplicates
    }
}
