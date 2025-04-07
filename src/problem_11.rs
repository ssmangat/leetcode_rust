pub struct Solution {}

impl Solution{
    pub fn container_with_most_water(height:Vec<i32>) -> i32{
        let mut max_area = 0;
        let mut left =  0;
        let mut right = height.len() as usize - 1;
        while left < right {
            let curr = (right-left) as i32 * height[left].min(height[right]);
            max_area = max_area.max(curr);
            if height[left] < height[right]{left += 1;}
            else {right -= 1}
        }
        max_area
    }
}
