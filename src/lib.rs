pub mod problem_1004;
pub mod problem_1071;
pub mod problem_11;
pub mod problem_1207;
pub mod problem_1431;
pub mod problem_1456;
pub mod problem_1493;
pub mod problem_151;
pub mod problem_1657;
pub mod problem_1732;
pub mod problem_1768;
pub mod problem_2215;
pub mod problem_238;
pub mod problem_283;
pub mod problem_345;
pub mod problem_392;
pub mod problem_605;
pub mod problem_643;
pub mod problem_724;
pub mod problem_75;

#[cfg(test)]
mod tests {

    mod problem_11_tests {
        use crate::problem_11::Solution;

        #[test]
        fn test_container_with_most_water() {
            assert_eq!(
                Solution::container_with_most_water(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]),
                49
            )
        }
    }

    mod problem_75_tests {
        use crate::problem_75::Solution;

        #[test]
        fn test_max_operations() {
            assert_eq!(
                Solution::max_operations(
                    vec![
                        63, 10, 28, 31, 90, 53, 75, 77, 72, 47, 45, 6, 49, 13, 77, 61, 68, 43, 33,
                        1, 14, 62, 55, 55, 38, 54, 8, 79, 89, 14, 50, 68, 85, 12, 42, 57, 4, 67,
                        75, 6, 71, 8, 61, 26, 11, 20, 22, 3, 70, 52, 82, 70, 67, 18, 66, 79, 84,
                        51, 78, 23, 19, 84, 46, 61, 63, 73, 80, 61, 15, 12, 58, 3, 21, 66, 42, 55,
                        7, 58, 85, 60, 23, 69, 41, 61, 35, 64, 58, 84, 61, 77, 45, 14, 1, 38, 4, 8,
                        42, 16, 79, 60, 80, 39, 67, 10, 24, 15, 6, 37, 68, 76, 30, 53, 63, 87, 11,
                        71, 86, 82, 77, 76, 37, 21, 85, 7, 75, 83, 80, 8, 19, 25, 11, 10, 41, 66,
                        70, 14, 23, 74, 33, 76, 35, 89, 68, 85, 83, 57, 6, 72, 34, 21, 57, 72, 79,
                        29, 65, 3, 67, 8, 24, 24, 18, 26, 27, 68, 78, 64, 57, 55, 68, 28, 9, 11,
                        38, 45, 61, 37, 81, 89, 38, 43, 45, 26, 84, 62, 22, 37, 51, 15, 30, 67, 75,
                        24, 66, 40, 81, 74, 48, 43, 78, 14, 33, 19, 73, 5, 1, 2, 53, 29, 49, 73,
                        23, 5
                    ],
                    59
                ),
                42
            );
        }
    }

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

    mod problem_238_test {
        use crate::problem_238::Solution;

        #[test]
        fn test_product_of_list() {
            assert_eq!(
                Solution::product_of_list(vec![-1, 1, 0, -3, 3]),
                vec![0, 0, 9, 0, 0]
            );
        }
    }

    mod problem_283_test {
        use crate::problem_283::Solution;

        #[test]
        fn test_move_zeros() {
            let mut v = vec![0, 1, 0, 12, 3, 0, 5];
            Solution::move_zeros(&mut v);
            assert_eq!(v, vec![1, 12, 3, 5, 0, 0, 0]);
        }
    }

    mod problem_392_tests {
        use crate::problem_392::Solution;

        #[test]
        fn test_is_subsequence() {
            assert_eq!(
                Solution::is_subsequence(String::from("abc"), String::from("ahbfcr")),
                true
            )
        }

        #[test]
        fn test_is_not_subsequence() {
            assert_ne!(
                Solution::is_subsequence(String::from("abcd"), String::from("ahbdfcr")),
                true
            )
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

    mod problem_643_tests {
        use crate::problem_643::Solution;

        #[test]
        fn test_maximum_average_subarray() {
            assert_eq!(
                Solution::maximum_average_subarray(
                    vec![
                        8860, -853, 6534, 4477, -4589, 8646, -6155, -5577, -1656, -5779, -2619,
                        -8604, -1358, -8009, 4983, 7063, 3104, -1560, 4080, 2763, 5616, -2375,
                        2848, 1394, -7173, -5225, -8244, -809, 8025, -4072, -4391, -9579, 1407,
                        6700, 2421, -6685, 5481, -1732, -8892, -6645, 3077, 3287, -4149, 8701,
                        -4393, -9070, -1777, 2237, -3253, -506, -4931, -7366, -8132, 5406, -6300,
                        -275, -1908, 67, 3569, 1433, -7262, -437, 8303, 4498, -379, 3054, -6285,
                        4203, 6908, 4433, 3077, 2288, 9733, -8067, 3007, 9725, 9669, 1362, -2561,
                        -4225, 5442, -9006, -429, 160, -9234, -4444, 3586, -5711, -9506, -79,
                        -4418, -4348, -5891
                    ],
                    93
                ),
                -594.58065
            )
        }
    }

    mod problem_724_tests {
        use crate::problem_724::Solution;

        #[test]
        fn test_find_pivot_index() {
            assert_eq!(Solution::find_pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        }
    }

    mod problem_1004_tests {
        use crate::problem_1004;

        #[test]
        fn test_max_consecutive_ones() {
            assert_eq!(
                problem_1004::Solution::max_consecutive_ones(
                    vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                    3
                ),
                10
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

    mod problem_1207_tests {
        use crate::problem_1207::Solution;

        #[test]
        fn test_unique_number_of_occurences() {
            assert_eq!(
                Solution::unique_number_of_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
                true
            );
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

    mod problem_1456_tests {
        use crate::problem_1456::Solution;

        #[test]
        fn test_maximum_vowels_in_substring() {
            assert_eq!(
                Solution::maximum_vowels_in_substring(String::from("leetcode"), 3),
                2
            );
        }
    }

    mod problem_1657_tests {
        use crate::problem_1657::Solution;

        #[test]
        fn test_close_strings() {
            assert_eq!(
                Solution::close_strings(String::from("cabbba"), String::from("abbccc")),
                true
            );
        }
    }

    mod problem_1493_tests {
        use crate::problem_1493::Solution;

        #[test]
        fn test_longest_subarray_after_removing_element() {
            assert_eq!(
                Solution::longest_subarray_after_removing_element(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
                5
            );
        }
    }

    mod problem_1732_tests {

        use crate::problem_1732::Solution;

        #[test]
        fn test_highest_altitude() {
            assert_eq!(Solution::highest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
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

    mod problem_2215_tests {
        use crate::problem_2215::Solution;

        #[test]
        fn test_difference_of_two_arrays() {
            assert_eq!(
                Solution::difference_of_two_arrays(vec![1, 2, 3], vec![2, 4, 6]),
                vec![vec![1, 3], vec![4, 6]]
            );
        }
    }
}
