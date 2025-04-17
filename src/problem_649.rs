pub struct Solution {}

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut r: Vec<usize> = Vec::new();
        let mut d: Vec<usize> = Vec::new();

        for (i, ch) in senate.chars().enumerate() {
            match ch {
                'R' => r.push(i),
                'D' => d.push(i),
                _ => (),
            }
        }

        while !r.is_empty() && !d.is_empty() {
            let r_index = r.remove(0);
            let d_index = d.remove(0);

            if r_index < d_index {
                r.push(r_index + senate.len());
            } else {
                d.push(d_index + senate.len());
            }
        }

        if !d.is_empty() {
            String::from("Dire")
        } else {
            String::from("Radiant")
        }
    }
}
