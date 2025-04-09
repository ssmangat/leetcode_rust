use std::collections::HashSet;
pub struct Solution{}

impl Solution{
    pub fn difference_of_two_arrays(nums1:Vec<i32>,nums2:Vec<i32>) -> Vec<Vec<i32>> {
        let s1: HashSet<i32> = nums1.iter().cloned().collect();
        let s2: HashSet<i32> = nums2.iter().cloned().collect();
        let l1: Vec<i32> = s1.difference(&s2).cloned().collect();
        let l2: Vec<i32> = s2.difference(&s1).cloned().collect();
        vec![l1,l2]
    }
}
