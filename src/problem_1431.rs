pub struct Candies {}

impl Candies {
    pub fn greatest_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result: Vec<bool> = Vec::new();
        let max_candies: i32 = *candies.iter().max().unwrap();
        for i in 0..candies.len() {
            let candies_this_iteration = candies.get(i).unwrap() + extra_candies;
            if candies_this_iteration < max_candies {
                result.push(false);
            } else {
                result.push(true);
            }
        }
        result
    }
}
