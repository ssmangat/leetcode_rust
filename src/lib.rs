pub mod problem_1768;

#[cfg(test)]
mod tests {

    mod problem_1768_tests {

        use crate::problem_1768::Solution;
        #[test]
        fn test_string_merge_equal_length() {
            let words = Solution {
                word1: String::from("lmno"),
                word2: String::from("pqrs"),
            };
            assert_eq!(words.merge_alternately(), "lpmqnros");
        }

        #[test]
        fn test_string_merge_unequal_length() {
            let words = Solution {
                word1: String::from("ab"),
                word2: String::from("defg"),
            };
            assert_eq!(words.merge_alternately(), "adbefg");
        }
    }
}
