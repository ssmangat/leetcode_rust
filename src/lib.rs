pub mod problem_1071;
pub mod problem_1431;
pub mod problem_151;
pub mod problem_1768;
pub mod problem_345;
pub mod problem_605;

#[cfg(test)]
mod tests {

    mod problem_151_test {
        use crate::problem_151::Solution;

        #[test]
        fn test_word_reversal_for_string() {
            assert_eq!(
                Solution::reverse_words_of_string(String::from(" Sky is yellow ,   NOT")),
                "NOT , yellow is Sky"
            );
        }
    }

    mod problem_345_tests {
        use crate::problem_345::Solution;

        #[test]
        fn test_reverse_vowels() {
            assert_eq!(
                Solution::reverse_vowels(String::from("sArtileumoyzet")),
                String::from("sertoluemiyzAt")
            );
        }
    }

    mod problem_605_tests {
        use crate::problem_605::FlowerBed;
        #[test]
        fn test_flower_bed() {
            assert_eq!(
                FlowerBed::can_place_flowers(vec![1, 0, 0, 0, 1, 0, 0], 2),
                true
            );
        }
    }
    mod problem_1071_tests {
        use crate::problem_1071::Gcd;

        #[test]
        fn test_strings_gcd_1() {
            assert_eq!(
                Gcd::gcd_strings("ABABAB".to_string(), "ABAB".to_string()),
                "AB".to_string()
            );
        }

        #[test]
        fn test_strings_gcd_2() {
            assert_eq!(
                Gcd::gcd_string_using_len("ABABAB".to_string(), "ABAB".to_string()),
                "AB"
            )
        }
    }

    mod problem_1431_tests {
        use crate::problem_1431::Candies;

        #[test]
        fn test_kids_with_candies() {
            assert_eq!(
                Candies::greatest_candies(vec![2, 3, 5, 1, 3], 3),
                vec![true, true, true, false, true]
            )
        }
    }

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
