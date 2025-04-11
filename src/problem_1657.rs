use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let s1: HashSet<char> = word1.chars().collect();
        let s2: HashSet<char> = word2.chars().collect();
        let s3: HashSet<char> = s1.union(&s2).cloned().collect();
        if s1.len() == s2.len() && s1.len() == s3.len() {
            let mut left = 0;
            let mut freq_word1 = Vec::new();
            let mut freq_word2 = Vec::new();
            for s in &s3 {
                freq_word1.insert(left, word1.chars().filter(|&c| c == *s).count());
                freq_word2.insert(left, word2.chars().filter(|&c| c == *s).count());
                left += 1;
            }
            freq_word2.sort();
            freq_word1.sort();
            return freq_word1 == freq_word2;
        }
        return false;
    }
}
