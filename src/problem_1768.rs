pub struct Solution {
    pub word1: String,
    pub word2: String,
}

impl Solution {
    pub fn merge_alternately(&self) -> String {
        let word_len = if self.word1.len() > self.word2.len() {
            self.word1.len()
        } else {
            self.word2.len()
        };
        let mut result = String::new();
        for i in 0..word_len {
            if let Some(c) = self.word1.chars().nth(i) {
                result.push(c);
            }
            if let Some(c) = self.word2.chars().nth(i) {
                result.push(c);
            }
        }
        result
    }
}
