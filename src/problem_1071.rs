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

    pub fn gcd_numbers(num1: usize, num2: usize) -> usize {
        if num2 == 0 {
            num1
        } else {
            Gcd::gcd_numbers(num2, num1 % num2)
        }
    }

    pub fn gcd_string_using_len(s1: String, s2: String) -> String {
        let len_s1 = s1.len();
        let len_s2 = s2.len();
        let gcd_len = Gcd::gcd_numbers(len_s1, len_s2);
        let mut result = String::new();
        for _ in 0..(gcd_len / len_s1) {
            result.push_str(&s1);
        }
        result.push_str(&s1[..(gcd_len % len_s1)]);

        result
    }
}
