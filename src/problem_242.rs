//problem_242 -> Valid anagram
//this problem provides us with 2 strings s and t.
//1. we check if the length of both is same otherwise we return false
//2. we initilize an array of 26 numbers in this case as 0;
//3. using zip we create a tuple from both s and t
//4. convert the characters to u8 and subtract value of letter a from it, this will give us correct
//   positions of those chars as alphabets for e.g. a at 0, b at 1;
//5. we add 1 for each value found in s and subtract 1 for each value found in t
//6. in the end we iterate over count array to see if there's any index that isn't 0, which would
//   mean that it's not a valid anagram
//

pub struct Solution {}

impl Solution {
    pub fn is_valid_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut count = [0; 26]; //array of size 26 for 26 alphabets
        for (s_char, t_char) in s.chars().zip(t.chars()) {
            count[(s_char as u8 - b'a') as usize] += 1;
            count[(t_char as u8 - b'a') as usize] -= 1;
        }
        count.iter().all(|&c| c == 0)
    }
}
