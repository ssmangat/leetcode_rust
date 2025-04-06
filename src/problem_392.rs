pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        if t.len() == 0 {
            return false;
        }
        let mut si = 0;
        let mut ti = 0;
        while si < s.len() && ti < t.len() {
            if s.chars().nth(si).unwrap() == t.chars().nth(ti).unwrap() {
                si += 1
            }
            ti += 1
        }
        si == s.len()
    }
}
