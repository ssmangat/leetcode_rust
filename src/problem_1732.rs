pub struct Solution {}

impl Solution{
    pub fn highest_altitude(gain:Vec<i32>)-> i32{
        let mut altitude = 0;
        let mut maxx = 0;
        for g in &gain {
            altitude += *g;
            maxx = maxx.max(altitude);
        }
        maxx
    }
}
