pub struct Solution {}

impl Solution {
    pub fn maximum_vowels_in_substring(s: String, k: i32) -> i32 {
        let k: usize = k as usize;
        let mut counter = 0;
        let mut current = 0;
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        for i in 0..k {
            if vowels.contains(&s.chars().nth(i).unwrap()) {
                current += 1;
            }
        }
        counter = counter.max(current);
        let mut left = 0;
        for i in k..s.len() {
            if vowels.contains(&s.chars().nth(left).unwrap()) {
                current -= 1;
            }
            left += 1;
            if vowels.contains(&s.chars().nth(i).unwrap()) {
                current += 1;
            }
            counter = counter.max(current);
        }
        counter
    }
}
