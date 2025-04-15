pub struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<String> = Vec::new();
        for ch in s.chars() {
            if ch != ']' {
                stack.push(ch.to_string());
            } else {
                let mut curr_str: String = String::new();
                while let Some(top) = stack.pop() {
                    if top == "[" {
                        break;
                    }
                    curr_str = top + &curr_str;
                }
                let mut curr_nums: String = String::new();
                while let Some(last) = stack.last() {
                    if last.chars().all(char::is_numeric) {
                        curr_nums = stack.pop().unwrap() + &curr_nums;
                    } else {
                        break;
                    }
                }
                let k = curr_nums.parse::<usize>().unwrap();
                curr_str = curr_str.repeat(k);
                stack.push(curr_str)
            }
        }
        stack.into_iter().collect::<String>()
    }
}
