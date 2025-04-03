pub struct Gcd {}

impl Gcd {
    pub fn gcd_strings(s1: String, s2: String) -> String {
        let s3 = s1.clone() + &s2;
        let s4 = s2.clone() + &s1;
        println!("S3 is {s3} and s4 is {s4}");
        if s3 != s4 {
            return "".to_string();
        }
        if s1.len() == s2.len() {
            return s1;
        }
        if s1.len() > s2.len() {
            return Gcd::gcd_strings(s1[s2.len()..].to_string(), s2);
        }
        return Gcd::gcd_strings(s1.clone(), s2[s1.len()..].to_string());
    }
}
