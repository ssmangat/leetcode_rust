pub struct Solution {}

impl Solution {
    pub fn reverse_words_of_string(s: String) -> String {
        s.split_whitespace()
            .rev()
            .map(|word| format!("{} ", word))
            .collect::<String>()
            .trim()
            .to_string()
    }
}
