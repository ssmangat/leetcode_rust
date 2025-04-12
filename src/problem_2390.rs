pub struct Solution{}

impl Solution {
    pub fn removing_stars_from_string(s:String) -> String {
        let mut stack = Vec::new();
        for c in s.chars(){
            if c == '*'{
                stack.pop();
            }else{
                stack.push(c);
            }
        }
        stack.into_iter().collect::<String>()
    }
}
