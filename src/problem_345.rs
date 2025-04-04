pub struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut max_index = s.len() - 1;
        let mut result: Vec<char> = s.chars().collect();
        let mut left = 0;

        fn vowel_check(c: char) -> bool {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false,
            }
        }

        while left < max_index {
            if !vowel_check(result[left]) {
                left += 1;
                continue;
            } else if !vowel_check(result[max_index]) {
                max_index -= 1;
                continue;
            }
            result.swap(left, max_index);
            left += 1;
            max_index -= 1;
        }

        result.iter().collect()
    }
}
