//problem 49 -> Group Anagram
//This problem includes a vector of strings, which could be Anagrams, we will return a vector
//containing vector of similar anagrams
//1. We will use hashmap for this
//2. HashMap will have keys as array of numbers for each word using bytes
//3. values will be added to each key based on if it's present or not.
use std::collections::HashMap;
pub struct Solution{}

impl Solution{
    pub fn group_anagram(strs:Vec<String>) -> Vec<Vec<String>>{
        let mut map: HashMap<[u8;26],Vec<String>> = HashMap::new();

        for word in strs {
            
            let mut count = [0u8;26];
            for ch in word.bytes(){
                count[(ch - b'a') as usize] += 1;
            }
            map.entry(count).or_default().push(word);
        }
        map.into_values().collect()
    }
}
