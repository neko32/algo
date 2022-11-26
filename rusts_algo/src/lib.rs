pub mod add_two_int_without_carry;
pub mod almost_increasing_seq;
pub mod appear_twice;
pub mod applebox;
pub mod arithmatic_progression;
pub mod array_product_sum;
pub mod binarysearch;
pub mod bintodec;
pub mod bt_from_inorder_preorder;
pub mod caesar_crypt;
pub mod camelcase;
pub mod century;
pub mod chars_to_sorted_digits;
pub mod christmas_tree;
pub mod clean_kth_bit;
pub mod combination;
pub mod construct_square;
pub mod cool_num_pair;
pub mod cyclic_chars;
pub mod decode_reverse_poland;
pub mod dectobin;
pub mod diagonal;
pub mod different_squares;
pub mod document_build;
pub mod euclidean;
pub mod even_num_sum;
pub mod factorial;
pub mod file_naming;
pub mod fizzbuzz;
pub mod geometric_progression;
pub mod helon_formula;
pub mod inorder_traversal;
pub mod is_mac_addr;
pub mod is_palindrome;
pub mod kadane;
pub mod lcs;
pub mod largest_adjacent_product;
pub mod largest_number;
pub mod least_data_eviction;
pub mod least_factorial;
pub mod levenshtein_distance;
pub mod linked_list;
pub mod max_sibling_product;
pub mod max_subset_sum;
pub mod max_with_lessdigit;
pub mod min_breakdown_sum;
pub mod min_reward;
pub mod minmax_stack;
pub mod number_grouping;
pub mod oppsite_pos_in_circle;
pub mod pascal_triangle;
pub mod permutation;
pub mod preorder_traversal;
pub mod product_array_sort;
pub mod radix_sort;
pub mod random_perm;
pub mod ranking;
pub mod reconstruct_bst_from_pre;
pub mod repeat_product;
pub mod request_per_sec;
pub mod reverse_poland_calc;
pub mod reverse_words;
pub mod rightmost_diffbit;
pub mod rightmost_samebit;
pub mod runlength;
pub mod shapearea;
pub mod selection_sort;
pub mod shared;
pub mod shorten_path;
pub mod sigma_k;
pub mod smallest_positive_product;
pub mod strange_bank;
pub mod string_construction;
pub mod string_pattern;
pub mod sum_of_arithmatic_progression;
pub mod swap_sibling;
pub mod tandem_repeat;
pub mod three_number_sort;
pub mod toggle_bit;
pub mod three_sum;
pub mod turn_commands;
pub mod two_sum;
pub mod waterarea;

use clap::Parser;

pub mod runner {

    use super::*;
    use add_two_int_without_carry::add_two_without_carry;
    use almost_increasing_seq::almost_increasing_seq;
    use appear_twice::appear_twice;
    use applebox::applebox;
    use arithmatic_progression;
    use array_product_sum::array_product_sum;
    use binarysearch::binary_search;
    use bintodec::bin_to_dec;
    use bt_from_inorder_preorder;
    use caesar_crypt::caesar_crypt;
    use camelcase::camelcase;
    use century::century;
    use chars_to_sorted_digits;
    use christmas_tree::christmas_tree;
    use clean_kth_bit::clean_kth_bit;
    use combination;
    use construct_square;
    use cool_num_pair::cool_num_pair;
    use cyclic_chars::cyclic_chars;
    use decode_reverse_poland::decode_reverse_poland;
    use dectobin::dectobin;
    use diagonal::diagonal;
    use different_squares;
    use document_build::document_build;
    use euclidean::euclidean;
    use even_num_sum::even_num_sum;
    use factorial;
    use file_naming;
    use fizzbuzz::fizzbuzz;
    use geometric_progression;
    use helon_formula::helon_formula;
    use inorder_traversal;
    use is_mac_addr::is_mac_addr;
    use is_palindrome::is_palindrome;
    use kadane::kadane;
    use largest_adjacent_product;
    use largest_number;
    use lcs::lcs;
    use least_data_eviction::least_data_eviction;
    use least_factorial::least_factorial;
    use levenshtein_distance;
    use linked_list;
    use max_sibling_product::max_sibling_product;
    use max_with_lessdigit::max_with_lessdigit;
    use min_breakdown_sum::min_breakdown_sum;
    use min_reward::min_reward;
    use minmax_stack;
    use number_grouping;
    use oppsite_pos_in_circle;
    use pascal_triangle;
    use permutation;
    use preorder_traversal::preorder_traversal;
    use product_array_sort::product_array_sort;
    use radix_sort::radix_sort;
    use random_perm;
    use ranking;
    use reconstruct_bst_from_pre;
    use repeat_product::repeat_product;
    use request_per_sec::request_per_sec;
    use reverse_poland_calc::reverse_poland_calc;
    use reverse_words;
    use rightmost_diffbit::rightmost_diffbit;
    use rightmost_samebit::rightmost_samebit;
    use runlength::runlength;
    use shapearea;
    use selection_sort::selection_sort;
    use shorten_path;
    use sigma_k::sigma_k;
    use smallest_positive_product::smallest_positive_product;
    use strange_bank;
    use string_construction::string_construction;
    use string_pattern::string_pattern;
    use sum_of_arithmatic_progression;
    use swap_sibling::swap_sibling;
    use tandem_repeat::tandem_repeat;
    use three_number_sort;
    use three_sum;
    use toggle_bit::toggle_bit;
    use turn_commands;
    use two_sum::two_sum;
    use waterarea::waterarea;

    pub fn exec(algo: Algo) {
        match algo {
            Algo::AddTwoIntWithoutCarry => {
                add_two_without_carry::run();
            }
            Algo::AlmostIncreasingSeq => {
                almost_increasing_seq::run();
            }
            Algo::AppearTwice => {
                appear_twice::run();
            }
            Algo::Applebox => {
                applebox::run();
            }
            Algo::ArithmaticProgression => {
                arithmatic_progression::run();
            }
            Algo::ArrayProductSum => {
                array_product_sum::run();
            }
            Algo::BinarySearch => {
                binary_search::run();
            }
            Algo::BinToDec => {
                bin_to_dec::run();
            }
            Algo::BuildBTreeFromInorderPreorder => {
                bt_from_inorder_preorder::run();
            }
            Algo::CaesarCrypt => {
                caesar_crypt::run();
            }
            Algo::CamelCase => {
                camelcase::run();
            }
            Algo::Century => {
                century::run();
            }
            Algo::CharsToSortedDigits => {
                chars_to_sorted_digits::run();
            }
            Algo::ChristmasTree => {
                christmas_tree::run();
            }
            Algo::CleanKthBit => {
                clean_kth_bit::run();
            }
            Algo::Combination => {
                combination::run();
            }
            Algo::ConstructSquare => {
                construct_square::run();
            }
            Algo::CoolNumPair => {
                cool_num_pair::run();
            }
            Algo::CyclicChars => {
                cyclic_chars::run();
            }
            Algo::DecodeReversePoland => {
                decode_reverse_poland::run();
            }
            Algo::DecToBin => {
                dectobin::run();
            }
            Algo::Diagonal => {
                diagonal::run();
            }
            Algo::DifferentSquares => {
                different_squares::run();
            }
            Algo::DocumentBuild => {
                document_build::run();
            }
            Algo::EvenNumSum => {
                even_num_sum::run();
            }
            Algo::Euclidean => {
                euclidean::run();
            }
            Algo::Factorial => {
                factorial::run();
            }
            Algo::FileNaming => {
                file_naming::run();
            }
            Algo::FizzBuzz => {
                fizzbuzz::run();
            }
            Algo::GeometricProgression => {
                geometric_progression::run();
            }
            Algo::HelonFormula => {
                helon_formula::run();
            }
            Algo::InOrderTraversal => {
                inorder_traversal::run();
            }
            Algo::IsPalindrome => {
                is_palindrome::run();
            }
            Algo::IsMacAddr => {
                is_mac_addr::run();
            }
            Algo::Kadane => {
                kadane::run();
            }
            Algo::LargestAdjacentProduct => {
                largest_adjacent_product::run();
            }
            Algo::LargestNumber => {
                largest_number::run();
            }
            Algo::LCS => {
                lcs::run();
            }
            Algo::LinkedList => {
                linked_list::run();
            }
            Algo::RadixSort => {
                radix_sort::run();
            }
            Algo::LeastDataEviction => {
                least_data_eviction::run();
            }
            Algo::LeastFactorial => {
                least_factorial::run();
            }
            Algo::LevenShteinDistance => {
                levenshtein_distance::run();
            }
            Algo::MaxSiblingProduct => {
                max_sibling_product::run();
            }
            Algo::MaxSubSetSum => {
                max_subset_sum::run();
            }
            Algo::MaxWithLessDigit => {
                max_with_lessdigit::run();
            }
            Algo::MinBreakdownSum => {
                min_breakdown_sum::run();
            }
            Algo::MinMaxStack => {
                minmax_stack::run();
            }
            Algo::MinReward => {
                min_reward::run();
            }
            Algo::NumberGrouping => {
                number_grouping::run();
            }
            Algo::OppositePosInCircle => {
                oppsite_pos_in_circle::run();
            }
            Algo::PascalTriangle => {
                pascal_triangle::run();
            }
            Algo::Permutation => {
                permutation::run();
            }
            Algo::PreOrderTraversal => {
                preorder_traversal::run();
            }
            Algo::ProductArraySort => {
                product_array_sort::run();
            }
            Algo::RandomPerm => {
                random_perm::run();
            }
            Algo::Ranking => {
                ranking::run();
            }
            Algo::ReconstructBSTFromPreorder => {
                reconstruct_bst_from_pre::run();
            }
            Algo::RepeatProduct => {
                repeat_product::run();
            }
            Algo::RequestPerSec => {
                request_per_sec::run();
            }
            Algo::ReversePoland => {
                reverse_poland_calc::run();
            }
            Algo::ReverseWords => {
                reverse_words::run();
            }
            Algo::RightMostDiffBit => {
                rightmost_diffbit::run();
            }
            Algo::RightMostSameBit => {
                rightmost_samebit::run();
            }
            Algo::RunLength => {
                runlength::run();
            }
            Algo::ShapeArea => {
                shapearea::run();
            }
            Algo::SelectionSort => {
                selection_sort::run();
            }
            Algo::ShortenPath => {
                shorten_path::run();
            }
            Algo::SigmaK => {
                sigma_k::run();
            }
            Algo::SmallestPositiveProduct => {
                smallest_positive_product::run();
            }
            Algo::StrangeBank => {
                strange_bank::run();
            }
            Algo::StringConstruction => {
                string_construction::run();
            }
            Algo::StringPattern => {
                string_pattern::run();
            }
            Algo::SumOfArithmaticProgression => {
                sum_of_arithmatic_progression::run();
            }
            Algo::SwapSibling => {
                swap_sibling::run();
            }
            Algo::TandemRepeat => tandem_repeat::run(),
            Algo::ThreeNumberSort => {
                three_number_sort::run();
            }
            Algo::ToggleBit => {
                toggle_bit::run();
            }
            Algo::ThreeSum => {
                three_sum::run();
            }
            Algo::TurnCommands => {
                turn_commands::run();
            }
            Algo::TwoSum => {
                two_sum::run();
            }
            Algo::WaterArea => {
                waterarea::run();
            }
        }
    }
}

#[cfg(test)]
mod test_runner {
    use crate::add_two_int_without_carry::add_two_without_carry;
    use crate::almost_increasing_seq::almost_increasing_seq;
    use crate::appear_twice::appear_twice;
    use crate::applebox::applebox;
    use crate::arithmatic_progression;
    use crate::array_product_sum::array_product_sum;
    use crate::binarysearch::binary_search;
    use crate::bintodec::bin_to_dec;
    use crate::bt_from_inorder_preorder;
    use crate::caesar_crypt::caesar_crypt;
    use crate::camelcase::camelcase;
    use crate::century::century;
    use crate::chars_to_sorted_digits;
    use crate::christmas_tree::christmas_tree;
    use crate::clean_kth_bit::clean_kth_bit;
    use crate::combination;
    use crate::construct_square;
    use crate::cool_num_pair::cool_num_pair;
    use crate::cyclic_chars::cyclic_chars;
    use crate::decode_reverse_poland::decode_reverse_poland;
    use crate::dectobin::dectobin;
    use crate::diagonal::diagonal;
    use crate::different_squares;
    use crate::document_build::document_build;
    use crate::euclidean::euclidean;
    use crate::even_num_sum::even_num_sum;
    use crate::factorial;
    use crate::file_naming;
    use crate::fizzbuzz::fizzbuzz;
    use crate::geometric_progression;
    use crate::helon_formula::helon_formula;
    use crate::inorder_traversal;
    use crate::is_mac_addr::is_mac_addr;
    use crate::is_palindrome::is_palindrome;
    use crate::kadane::kadane;
    use crate::largest_adjacent_product;
    use crate::largest_number;
    use crate::lcs::lcs;
    use crate::least_data_eviction::least_data_eviction;
    use crate::least_factorial::least_factorial;
    use crate::levenshtein_distance;
    use crate::linked_list;
    use crate::max_sibling_product::max_sibling_product;
    use crate::max_subset_sum;
    use crate::max_with_lessdigit::max_with_lessdigit;
    use crate::min_breakdown_sum::min_breakdown_sum;
    use crate::min_reward::min_reward;
    use crate::minmax_stack;
    use crate::number_grouping;
    use crate::pascal_triangle;
    use crate::permutation;
    use crate::preorder_traversal::preorder_traversal;
    use crate::product_array_sort::product_array_sort;
    use crate::oppsite_pos_in_circle;
    use crate::radix_sort::radix_sort;
    use crate::random_perm;
    use crate::ranking;
    use crate::reconstruct_bst_from_pre;
    use crate::repeat_product::repeat_product;
    use crate::request_per_sec::request_per_sec;
    use crate::reverse_poland_calc::reverse_poland_calc;
    use crate::reverse_words;
    use crate::rightmost_diffbit::rightmost_diffbit;
    use crate::rightmost_samebit::rightmost_samebit;
    use crate::runlength::runlength;
    use crate::selection_sort::selection_sort;
    use crate::shapearea;
    use crate::shared::*;
    use crate::shorten_path;
    use crate::sigma_k::sigma_k;
    use crate::smallest_positive_product::smallest_positive_product;
    use crate::strange_bank;
    use crate::string_construction::string_construction;
    use crate::string_pattern::string_pattern;
    use crate::sum_of_arithmatic_progression;
    use crate::swap_sibling::swap_sibling;
    use crate::tandem_repeat::tandem_repeat;
    use crate::three_number_sort;
    use crate::three_sum;
    use crate::toggle_bit::toggle_bit;
    use crate::turn_commands;
    use crate::two_sum::two_sum;
    use crate::waterarea::waterarea;
    use float_cmp::approx_eq;
    use num::integer::gcd;
    use std::{
        array,
        collections::{BTreeSet, HashSet},
    };

    #[test]
    fn add_two_int_without_carry_test() {
        let a = 456;
        let b = 1734;
        assert_eq!(add_two_without_carry::exec(a, b), 1180);
    }

    #[test]
    fn almost_increasing_seq_false_case() {
        let s = [1, 3, 2, 1];
        assert_ne!(true, almost_increasing_seq::exec(&s));
    }

    #[test]
    fn almost_increasing_seq_true_case() {
        let s = [1, 2, 3, 4, 5, 5, 7, 6, 8];
        assert!(almost_increasing_seq::exec(&s));
    }

    #[test]
    fn appear_twice_test() {
        let expected: BTreeSet<char> = BTreeSet::from_iter(['b', 'd']);
        let rez = appear_twice::exec();
        assert_eq!(expected, rez);
    }

    #[test]
    fn applebox_test() {
        let n = 5;
        let expected = -15;
        assert_eq!(applebox::exec(n), expected);
    }

    #[test]
    fn arithmatic_progression_test() {
        assert_eq!(arithmatic_progression::exec(1, 5, 2), 5);
        assert_eq!(arithmatic_progression::exec(3, 5, 2), 9);
        assert_eq!(arithmatic_progression::exec(5, 1, 6), 25);
    }

    #[test]
    fn array_product_sum_test() {
        let n = [5, 1, 4, 2];
        let p = &n[..];
        let r = array_product_sum::exec(p);
        assert_eq!(r, vec![8, 40, 10, 20]);
    }

    #[test]
    fn binsearch_test_found() {
        let s = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
        let len = s.len();
        let b: Box<Vec<i32>> = Box::new(s);
        let r = binary_search::exec(&b, 33, 0, len - 1);
        assert_eq!(r, 3);
    }

    #[test]
    fn binsearch_test_notfound() {
        let s = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
        let len = s.len();
        let b: Box<Vec<i32>> = Box::new(s);
        let r = binary_search::exec(&b, 50, 0, len - 1);
        assert_eq!(r, -1);
    }

    #[test]
    fn bin_to_dec_test1() {
        let b = "10001010";
        let r = bin_to_dec::exec(b);
        assert_eq!(r, 138);
    }

    #[test]
    fn bin_to_dec_test2() {
        let b = "01001111";
        let r = bin_to_dec::exec(b);
        assert_eq!(r, 79);
    }

    #[test]
    fn build_btree_from_preorder_inorder_test() {
        let preorder = [3, 9, 20, 15, 7];
        let inorder = [9, 3, 15, 20, 7];
        match bt_from_inorder_preorder::exec(&preorder, &inorder) {
            Some(bn) => {
                let mut buf:Vec<i32> = Vec::new();
                traverse(bn, &mut buf);
                let mut expected:Vec<i32> = Vec::new();
                expected.extend_from_slice(&inorder);
                assert_eq!(buf, expected);
            },
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn caesar_crypt_test() {
        let s = "TAkonbiz";
        let n = 3;
        assert_eq!("wdnrqelc", caesar_crypt::exec(s, n));
    }

    #[test]
    fn camelcase_test() {
        let c = camelcase::exec("taKOcHaN".to_string());
        assert_eq!(c, "Takochan".to_string());
    }

    #[test]
    fn century_test1() {
        let n = 1905;
        let nr = century::exec(n);
        assert_eq!(nr, 20);
    }

    #[test]
    fn century_test2() {
        let n = 1700;
        let nr = century::exec(n);
        assert_eq!(nr, 17);
    }

    #[test]
    fn chars_to_sorted_digits_test() {
        assert_eq!(chars_to_sorted_digits::exec("owoztneoer"), "012".to_string());
        assert_eq!(chars_to_sorted_digits::exec("fviefuro"), "45".to_string());
    }

    #[test]
    fn christmastree_test() {
        let t = [
            "       *  ",
            "     *    ",
            "***       ",
            "     *****",
            "   *******",
            "**********",
            " ***      ",
        ];
        let rez = christmas_tree::exec(&t);
        let rezstr: String = rez.join("\n");
        let expected = r"-----------
|    *    |
|    *    |
|   ***   |
|  *****  |
| ******* |
|*********|
|   ***   |
-----------";
        assert_eq!(rezstr, expected);
    }

    #[test]
    fn clean_kth_bit_test() {
        let n = 127;
        let k = 3;
        assert_eq!(clean_kth_bit::exec(n, k), 123);
    }

    #[test]
    fn combination_test() {
        let n = 8;
        let r = 3;
        assert_eq!(combination::exec(n, r), 56);
    }

    #[test]
    fn construct_square_simple_good_test() {
        let s = "aba".to_string();
        let rez = construct_square::exec(&s);
        assert_eq!(rez, 900);
    }

    #[test]
    fn consturst_square_bad_case_test() {
        let s = "zzz".to_string();
        let rez = construct_square::exec(&s);
        assert_eq!(rez, -1);
    }

    #[test]
    fn construct_square_bignum_good_test() {
        let s = "aaaabbcde".to_string();
        let rez = construct_square::exec(&s);
        assert_eq!(rez, 999950884);
    }

    #[test]
    fn cool_num_pair() {
        let a = [4, 5, 6, 7, 8];
        let b = [8, 9, 10, 11, 12];
        assert_eq!(cool_num_pair::exec(&a, &b), 2);
    }

    #[test]
    fn cyclic_chars_test() {
        let n = 7;
        let s = "ABC";
        assert_eq!(cyclic_chars::exec(s, n), "ABCABCA");
    }

    #[test]
    fn decode_reverse_poland_test() {
        let s = "612+*8-";

        let expected = "6*(1+2)-8";
        assert_eq!(decode_reverse_poland::exec(s), expected);
    }

    #[test]
    fn different_sq_test() {
        let v:Vec<Vec<i32>> = vec![
            vec![1, 2, 1],
            vec![2, 2, 2],
            vec![2, 2, 2],
            vec![1, 2, 3],
            vec![2 ,2, 1],
        ];
        assert_eq!(different_squares::exec(v), 6);
    }

    #[test]
    fn even_num_sum_match_case() {
        let n = 1230;
        assert!(even_num_sum::exec(n));
    }

    #[test]
    fn even_num_sum_unmatch_case() {
        let n = 124568;
        assert_eq!(false, even_num_sum::exec(n));
    }

    #[test]
    fn euclidean_test1() {
        let m = 128;
        let n = 72;
        let gcd = gcd(m, n);
        assert_eq!(euclidean::exec(m, n), gcd);
    }

    #[test]
    fn euclidean_test2() {
        let m = 72;
        let n = 128;
        let gcd = gcd(m, n);
        assert_eq!(euclidean::exec(m, n), gcd);
    }

    #[test]
    fn factorial_test() {
        let n = 10;
        assert_eq!(factorial::exec(n), 3628800);
    }

    #[test]
    fn file_naming_test() {
        let files = vec!["doc", "doc", "image", "doc(1)", "doc"];
        let expected = vec!["doc".to_string(), "doc(1)".to_string(), "image".to_string(), "doc(1)(1)".to_string(), "doc(2)".to_string()];
        assert_eq!(file_naming::exec(files), expected);
    }

    #[test]
    fn test_fizzbuzz() {
        let n = 30;
        let rez = fizzbuzz::exec(n);
        let expected:String = "12fizz4buzzfizz78fizzbuzz11fizz1314fizzbuzz1617fizz19buzzfizz2223fizzbuzz26fizz2829fizzbuzz".to_string();
        assert_eq!(rez, expected);
    }

    #[test]
    fn geometric_progression_test() {
        assert_eq!(geometric_progression::exec(1, 2, 3), 2);
        assert_eq!(geometric_progression::exec(3, 2, 3), 18);
        assert_eq!(geometric_progression::exec(4, 2, 3), 54);
        assert_eq!(geometric_progression::exec(5, 2, 3), 162);
    }

    #[test]
    fn helon_formula_test() {
        let a = 5.0;
        let b = 6.0;
        let c = 7.0;
        let expect = 14.6969385;
        let rez = approx_eq!(f32, expect, helon_formula::exec(a, b, c), ulps = 2);
        assert!(rez);
    }

    #[test]
    fn inorder_trav_test() {
        let v = vec![5, 9, 2, 10, 1, 4];
        let r = inorder_traversal::exec(&v);
        let expected = vec![1, 2, 4, 5, 9, 10];
        assert_eq!(r, expected);
    }

    #[test]
    fn is_mac_addr_good() {
        let mac = "00-1B-63-84-45-E6";
        assert_eq!(is_mac_addr::exec(mac), true);
    }

    #[test]
    fn is_palindrome_good_evencase() {
        let s = "abccba";
        assert_eq!(is_palindrome::exec(&s.to_string()), true);
    }

    #[test]
    fn is_palindrome_good_oddcase() {
        let s = "abcba";
        assert_eq!(is_palindrome::exec(&s.to_string()), true);
    }

    #[test]
    fn is_palindrome_badcase() {
        let s = "abaya";
        assert_eq!(is_palindrome::exec(&s.to_string()), false);
    }

    #[test]
    fn is_mac_addr_bad() {
        let mac = "00-1B-63-84-45-Z6";
        assert_eq!(is_mac_addr::exec(mac), false);
    }

    #[test]
    fn kadane_test() {
        let v = vec![3, 5, -9, 1, 3, -2, 3, 4, 7, 2, -9, 6, 3, 1, -5, 4];
        assert_eq!(19, kadane::exec(v));
    }

    #[test]
    fn largest_adjacent_product_many_elem_test() {
        let v = [3, 6, -2, -5, 7, 3];
        assert_eq!(largest_adjacent_product::exec(&v), 21);
    }

    #[test]
    fn largest_adjacent_product_single_elem_test() {
        let v= [0];
        assert_eq!(largest_adjacent_product::exec(&v), 0);
    }

    #[test]
    fn largest_adjacent_product_no_elem_test() {
        let v:&[i32] = &[];
        let maybe_e = std::panic::catch_unwind(||{
            largest_adjacent_product::exec(v);
        });
        assert!(maybe_e.is_err());
    }

    #[test]
    fn largest_number_test() {
        let expected:Vec<u64> = vec![0, 9, 99, 999, 9999, 99999, 999999,
        9999999, 99999999, 999999999, 9999999999];
        for i in 1..=10 {
            let r:u64 = largest_number::exec(i);
            assert_eq!(r, expected[i as usize]);
        }
    }

    #[test]
    fn lcs_test1() {
        let m = "abcde";
        let n = "acbef";
        assert_eq!(lcs::exec(m, n), 3);
    }

    #[test]
    fn least_data_eviction_test() {
        let seq = vec![12, 4, 5, 19, 20, 10, 6, 2, 1, 33, 22, 25, 16, 7, 4, 20, 15, 30, 2, 7];
        let rez = least_data_eviction::exec(&seq);
        assert_eq!(rez.len(), 3);
        assert_eq!(rez, vec![25, 30, 33]);
    }

    #[test]
    fn least_factorial_test() {
        let n = 17;
        assert_eq!(least_factorial::exec(n), 24);
    }

    #[test]
    fn levenshtein_distance_test() {
        let s = "abc";
        let t = "yabd";
        let diff = levenshtein_distance::exec(s, t);
        assert_eq!(diff, 2);
    }

    #[test]
    fn linked_list_forward_trav_test() {
        let v = &[1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(linked_list::exec(v), expected);
    }

    #[test]
    fn max_sibling_product_test() {
        let s = vec![3, 6, -2, -5, 7, 3];
        let r = max_sibling_product::exec(s);
        assert_eq!(r, 21);
    }

    #[test]
    fn max_subset_sum_test() {
        let a = [75, 105, 120, 75, 90, 135];
        assert_eq!(max_subset_sum::exec(&a), 330);
    }

    #[test]
    fn max_with_lessdigit_test1() {
        let n = 1001;
        let maxv = max_with_lessdigit::exec(n);
        assert_eq!(maxv, 101);
    }

    #[test]
    fn max_with_lessdigit_test2() {
        let n = 597;
        let maxv = max_with_lessdigit::exec(n);
        assert_eq!(maxv, 97);
    }

    #[test]
    fn min_breakdown_sum_test1() {
        let n:u32 = 240;
        let rez = min_breakdown_sum::exec(n);
        assert_eq!(rez, 4);
    }

    #[test]
    fn min_breakdown_sum_test2() {
        let n:u32 = 808;
        let rez = min_breakdown_sum::exec(n);
        assert_eq!(rez, 14);
    }

    #[test]
    fn min_reward_test() {
        let scores = [8, 4, 2, 1, 3, 6, 7, 9, 5];
        assert_eq!(min_reward::exec(&scores), 25);
    }

    #[test]
    fn minmax_stack_test() {
        let maybe_err = std::panic::catch_unwind(||{
            minmax_stack::run();
        });
        assert_ne!(maybe_err.is_err(), true);
    }

    #[test]
    fn number_grouping_test() {
        let a = vec![10000, 20000, 30000, 40000, 50000, 60000, 10000, 120000, 150000, 200000, 300000, 1000000, 10000000, 100000000, 10000000];
        assert_eq!(number_grouping::exec(&a), 28);
    }

    #[test]
    fn opposite_num_in_circle_test() {
        let n = 10;
        let f = 2;
        assert_eq!(oppsite_pos_in_circle::exec(n, f), 7);
    }

    #[test]
    fn lcs_test2() {
        let m = "pirikapirirara";
        let n = "poporinapeperuto";
        assert_eq!(lcs::exec(m, n), 6);
    }

    #[test]
    fn radix_sort_test() {
        let mut v = vec![8762, 654, 3008, 345, 87, 65, 234, 12, 2];
        radix_sort::exec(&mut v);
        assert_eq!(vec![2, 12, 65, 87, 234, 345, 654, 3008, 8762], v);
    }

    #[test]
    fn pascal_tr_test() {
        let n = 5;
        let expected = r"1
1 1
1 2 1
1 3 3 1
1 4 6 4 1";
        let rez = pascal_triangle::exec(n);
        assert_eq!(rez.trim(), expected);
    }

    #[test]
    fn perm_test() {
        let n = 8;
        let r = 6;
        assert_eq!(permutation::exec(n, r), 20160);
    }

    #[test]
    fn preorder_trav_test() {
        let v: Vec<i32> = vec![5, 9, 2, 10, 1, 4];
        let r: Vec<i32> = preorder_traversal::exec(v);
        assert_eq!(r, vec![5, 2, 1, 4, 9, 10]);
    }

    #[test]
    fn rand_perm_test() {
        let o:Vec<i32> = (0..=10).collect();
        let mut v:Vec<i32> = (0..=10).collect();
        random_perm::exec(&mut v);
        assert_ne!(v, o);
    }

    #[test]
    fn ranking_test() {
        let scores = vec![56, 25, 67, 88, 100, 61, 55, 67, 76, 56];
        let expected:Vec<(i32, u32)> = vec![(56, 7), (25, 10), (67, 4), (88, 2), (100, 1), (61, 6), (55, 9), (67, 4), (76, 3), (56, 7)];
        assert_eq!(ranking::exec(&scores), expected);
    }

    #[test]
    fn reconstruct_bst_from_preorder_test() {
        let v:Vec<i32> = vec![10, 4, 2, 1, 5, 17, 19, 18];
        let Some(rez) = reconstruct_bst_from_pre::exec(&v) else {
            panic!("failed");
        };
        let mut trace:Vec<i32> = Vec::new();
        traverse_pre(rez, &mut trace);
        assert_eq!(v, trace);
    }

    #[test]
    fn repeat_product_test() {
        let n = 16;
        assert_eq!(repeat_product::exec(n), 9);
    }

    #[test]
    fn req_per_sec_test() {
        let n = 17;
        let rez = request_per_sec::exec(n);
        println!("{}-{}", rez, 928);
        assert!(true);
    }

    #[test]
    fn reverse_poland_test1() {
        let s = "12+3+4+";
        assert_eq!(reverse_poland_calc::exec(s), 10_f32);
    }

    #[test]
    fn reverse_poland_test2() {
        let s = "374/-8*";
        assert_eq!(reverse_poland_calc::exec(s), 10_f32);
    }

    #[test]
    fn reverse_words_test() {
        let r = "nekochan kawaii to omoimasenka? soudesu.";
        let expected:String = "soudesu. omoimasenka? to kawaii nekochan".to_string();
        assert_eq!(reverse_words::exec(r), expected);
    }

    #[test]
    fn runlength_test1() {
        let s = "AAAAAAAAAAAA".to_string();
        let rez = runlength::exec(s);
        assert_eq!(rez, "9A3A".to_string());
    }

    #[test]
    fn runlength_test2() {
        let s = "AABBBCDD".to_string();
        let rez = runlength::exec(s);
        assert_eq!(rez, "2A3B1C2D".to_string());
    }

    #[test]
    fn shapearea_test() {
        assert_eq!(shapearea::exec(1), 1);
        assert_eq!(shapearea::exec(2), 5);
        assert_eq!(shapearea::exec(3), 13);
        assert_eq!(shapearea::exec(7000), 97986001);
    }

    #[test]
    fn shorten_path_absolute_path_case() {
        let path = "/foo/../test/../test/../foo//bar/./baz";
        let expected = "/foo/bar/baz".to_string();
        assert_eq!(shorten_path::exec(path), expected);
    }

    #[test]
    fn shorten_path_relative_path_case() {
        let path = "../tako/../test/../test/./baz";
        let expected = "test/baz".to_string();
        assert_eq!(shorten_path::exec(path), expected);
    }

    #[test]
    fn sigma_k_test() {
        let k = 7;
        let sigma_k = sigma_k::exec(k);
        let sum: i32 = (1..=k).sum();
        assert_eq!(sigma_k, sum);
    }

    #[test]
    fn smallest_positive_good_case() {
        let n = 12;
        assert_eq!(smallest_positive_product::exec(n), 26);
    }

    #[test]
    fn smallest_positive_bad_case() {
        let n = 19;
        assert_eq!(smallest_positive_product::exec(n), -1);
    }

    #[test]
    fn strange_bank_ok_test() {
        let n = 7_u32;
        let denoms = &[1, 5, 10];
        let rez = strange_bank::exec(n, denoms);
        assert_eq!(rez, 3);
    }

    #[test]
    fn strange_bank_nochange_made_test() {
        let n = 9_u32;
        let denoms = &[4, 6, 10, 50];
        let rez = strange_bank::exec(n, denoms);
        assert_eq!(rez, -1);
    }

    #[test]
    fn string_construction_test1() {
        let a = "abc";
        let b = "abccba";
        assert_eq!(string_construction::exec(a, b), 2);
    }

    #[test]
    fn string_construction_test2() {
        let a = "abc";
        let b = "abba";
        assert_eq!(string_construction::exec(a, b), 0);
    }

    #[test]
    fn string_pattern_test_simplecase() {
        let input = "21";
        let expected = "1211";
        assert_eq!(string_pattern::exec(input).as_str(), expected);
    }

    #[test]
    fn string_pattern_test_complexcase() {
        let input = "33372211115";
        let expected = "3317224115";
        assert_eq!(string_pattern::exec(input).as_str(), expected);
    }

    #[test]
    fn sum_of_arithmatic_progression_test() {
        assert_eq!(sum_of_arithmatic_progression::exec(4, 5, 4, 3), 34);
        assert_eq!(sum_of_arithmatic_progression::exec(5, 5, 4, 3), 50);
    }

    #[test]
    fn tandem_repeat_case1() {
        let c = "CatCat".to_string();
        assert!(tandem_repeat::exec(c));
    }

    #[test]
    fn dec_to_bin_test1() {
        let n = 23;
        let b = dectobin::exec(n);
        assert_eq!(b, "10111".to_string());
    }

    #[test]
    fn three_sum_test() {
        let mut a = vec![12, 3, 1, 2, -6, 5, -8, 6];
        let target = 0;
        let rez = three_sum::exec(&mut a, target);
        assert_eq!(rez.len(), 3);
        assert_eq!(rez[0], vec![-8, 2, 6]);
        assert_eq!(rez[1], vec![-8, 3, 5]);
        assert_eq!(rez[2], vec![-6, 1, 5]);
    }

    #[test]
    fn diagonal_is_diag_yes() {
        let a = Point::new(1, 5);
        let b = Point::new(9, 13);
        println!(
            "a:{},b:{} must be diagonal - result is {}",
            a,
            b,
            diagonal::exec(&a, &b)
        );
        assert!(diagonal::exec(&a, &b));
    }


    #[test]
    fn diagonal_is_diag_no() {
        let a = Point::new(3, 5);
        let b = Point::new(8, 5);
        println!(
            "a:{},b:{} must be diagonal - result is {}",
            a,
            b,
            diagonal::exec(&a, &b)
        );
        assert!(diagonal::exec(&a, &b) == false);
    }

    #[test]
    fn document_build_test() {
        let s = "Todd told Tom to trot to the timber".to_string();
        let expected: Vec<char> = vec![' ', 'T', 'b', 'd', 'e', 'h', 'i', 'l', 'm', 'o', 'r', 't'];
        assert_eq!(document_build::exec(s), expected);
    }

    #[test]
    fn product_array_sort_test() {
        let v = vec![-11, -6, 0, 5, 8, 10];
        let w = product_array_sort::exec(&v);
        assert_eq!(w, vec![0, 25, 36, 64, 100, 121]);
    }

    #[test]
    fn rightmost_diffbit_case1() {
        let m = 13;
        let n = 11;
        assert_eq!(2, rightmost_diffbit::exec(m, n));
    }

    #[test]
    fn rightmost_diffbit_case2() {
        let m = 7;
        let n = 23;
        assert_eq!(16, rightmost_diffbit::exec(m, n));
    }

    #[test]
    fn rightmost_samebit_case1() {
        let m = 10;
        let n = 11;
        assert_eq!(2, rightmost_samebit::exec(m, n));
    }

    #[test]
    fn swap_sibling_test() {
        let mut v: Vec<i32> = (1..=6).collect();
        swap_sibling::exec(&mut v);
        let expected: Vec<i32> = vec![2, 1, 4, 3, 6, 5];
        assert_eq!(v, expected);
    }

    #[test]
    fn tandem_repeat_case2() {
        let a = "ABA".to_string();
        let b = "cattac".to_string();
        assert!(!tandem_repeat::exec(a) && !tandem_repeat::exec(b));
    }

    #[test]
    fn three_number_sort_test() {
        let order = [0, 1, -1];
        let mut arr = [1, 0, 0, -1, -1, 0, 1, 1];
        let expected = [0, 0, 0, 1, 1, 1, -1, -1];
        three_number_sort::exec(&mut arr, &order);
        assert_eq!(arr, expected);
    }

    #[test]
    fn toggle_bit_test() {
        let n = 15;
        let k = 2;
        let rez1 = toggle_bit::exec(n, k);
        assert_eq!(rez1, 13);
        let rez2 = toggle_bit::exec(rez1, k);
        assert_eq!(rez2, n);
    }

    #[test]
    fn turn_commands_test() {
        let cmd = "LLARL";
        assert_eq!(turn_commands::exec(cmd), 3);
    }

    #[test]
    fn two_sum_case1() {
        let rez = two_sum::exec(vec![3, 5, -4, 8, 11, 1, -1, 6], 10);
        assert_eq!(rez, vec![-1, 11])
    }

    #[test]
    fn two_sum_case2() {
        let rez = two_sum::exec(vec![4, 6], 10);
        let rset: HashSet<i32> = HashSet::from_iter(rez);
        let expected: HashSet<i32> = HashSet::from_iter([4, 6]);
        assert_eq!(rset, expected);
    }

    #[test]
    fn two_sum_case3() {
        let rez = two_sum::exec(vec![-21, 301, 12, 4, 65, 56, 210, 356, 9, -47], 163);
        let rset: HashSet<i32> = HashSet::from_iter(rez);
        let expected: HashSet<i32> = HashSet::from_iter([210, -47]);
        assert_eq!(rset, expected);
    }

    #[test]
    fn sel_sort_test() {
        let mut n = vec![1, 9, 4, 10, -3, 9, 15, 2];
        selection_sort::exec(&mut n);
        assert_eq!(n, vec![-3, 1, 2, 4, 9, 9, 10, 15]);
    }

    #[test]
    fn waterarea_test() {
        let a: Vec<u32> = vec![0, 8, 0, 0, 5, 0, 0, 10, 0, 0, 1, 1, 0, 3];
        let r = waterarea::exec(a);
        assert_eq!(r, 48);
    }
}

pub enum Algo {
    AddTwoIntWithoutCarry,
    AlmostIncreasingSeq,
    AppearTwice,
    Applebox,
    ArithmaticProgression,
    ArrayProductSum,
    BinarySearch,
    BinToDec,
    BuildBTreeFromInorderPreorder,
    CamelCase,
    Century,
    CaesarCrypt,
    CharsToSortedDigits,
    ChristmasTree,
    CleanKthBit,
    Combination,
    ConstructSquare,
    CoolNumPair,
    CyclicChars,
    DecodeReversePoland,
    DecToBin,
    Diagonal,
    DifferentSquares,
    DocumentBuild,
    EvenNumSum,
    Euclidean,
    Factorial,
    FileNaming,
    FizzBuzz,
    GeometricProgression,
    HelonFormula,
    InOrderTraversal,
    IsMacAddr,
    IsPalindrome,
    Kadane,
    LargestAdjacentProduct,
    LargestNumber,
    LCS,
    LeastDataEviction,
    LeastFactorial,
    LevenShteinDistance,
    LinkedList,
    MaxSiblingProduct,
    MaxSubSetSum,
    MaxWithLessDigit,
    MinBreakdownSum,
    MinMaxStack,
    MinReward,
    NumberGrouping,
    OppositePosInCircle,
    PascalTriangle,
    Permutation,
    PreOrderTraversal,
    ProductArraySort,
    RadixSort,
    RandomPerm,
    Ranking,
    ReconstructBSTFromPreorder,
    RepeatProduct,
    RequestPerSec,
    ReversePoland,
    ReverseWords,
    RightMostDiffBit,
    RightMostSameBit,
    RunLength,
    ShapeArea,
    SelectionSort,
    ShortenPath,
    SigmaK,
    SmallestPositiveProduct,
    StrangeBank,
    StringConstruction,
    StringPattern,
    SumOfArithmaticProgression,
    SwapSibling,
    TandemRepeat,
    ThreeNumberSort,
    ThreeSum,
    ToggleBit,
    TurnCommands,
    TwoSum,
    WaterArea,
}

impl Algo {
    pub fn from_str(algo_str: &str) -> Self {
        match algo_str {
            s if s.to_lowercase() == "add_two_int_without_carry" => Algo::AddTwoIntWithoutCarry,
            s if s.to_lowercase() == "almost_increasing_seq" => Algo::AlmostIncreasingSeq,
            s if s.to_lowercase() == "appear_twice" => Algo::AppearTwice,
            s if s.to_lowercase() == "applebox" => Algo::Applebox,
            s if s.to_lowercase() == "arithmatic_progression" => Algo::ArithmaticProgression,
            s if s.to_lowercase() == "array_product_sum" => Algo::ArrayProductSum,
            s if s.to_lowercase() == "binary_search" => Algo::BinarySearch,
            s if s.to_lowercase() == "bintodec" => Algo::BinToDec,
            s if s.to_lowercase() == "build_bt_from_preorder_inorder" => Algo::BuildBTreeFromInorderPreorder,
            s if s.to_lowercase() == "caesar_crypt" => Algo::CaesarCrypt,
            s if s.to_lowercase() == "camelcase" => Algo::CamelCase,
            s if s.to_lowercase() == "century" => Algo::Century,
            s if s.to_lowercase() == "chars_to_sorted_digits" => Algo::CharsToSortedDigits,
            s if s.to_lowercase() == "christmas_tree" => Algo::ChristmasTree,
            s if s.to_lowercase() == "clean_kth_bit" => Algo::CleanKthBit,
            s if s.to_lowercase() == "combination" => Algo::Combination,
            s if s.to_lowercase() == "construct_square" => Algo::ConstructSquare,
            s if s.to_lowercase() == "cool_num_pair" => Algo::CoolNumPair,
            s if s.to_lowercase() == "cyclic_chars" => Algo::CyclicChars,
            s if s.to_lowercase() == "decode_reverse_poland" => Algo::DecodeReversePoland,
            s if s.to_lowercase() == "dectobin" => Algo::DecToBin,
            s if s.to_lowercase() == "diagonal" => Algo::Diagonal,
            s if s.to_lowercase() == "different_squares" => Algo::DifferentSquares,
            s if s.to_lowercase() == "document_build" => Algo::DocumentBuild,
            s if s.to_lowercase() == "even_num_sum" => Algo::EvenNumSum,
            s if s.to_lowercase() == "euclidean" => Algo::Euclidean,
            s if s.to_lowercase() == "factorial" => Algo::Factorial,
            s if s.to_lowercase() == "file_naming" => Algo::FileNaming,
            s if s.to_lowercase() == "fizzbuzz" => Algo::FizzBuzz,
            s if s.to_lowercase() == "geometric_progression" => Algo::GeometricProgression,
            s if s.to_lowercase() == "helon_formula" => Algo::HelonFormula,
            s if s.to_lowercase() == "inorder_traversal" => Algo::InOrderTraversal,
            s if s.to_lowercase() == "is_palindrome" => Algo::IsPalindrome,
            s if s.to_lowercase() == "is_mac_addr" => Algo::IsMacAddr,
            s if s.to_lowercase() == "kadane" => Algo::Kadane,
            s if s.to_lowercase() == "largest_adjacent_product" => Algo::LargestAdjacentProduct,
            s if s.to_lowercase() == "largest_number" => Algo::LargestNumber,
            s if s.to_lowercase() == "lcs" => Algo::LCS,
            s if s.to_lowercase() == "least_data_eviction" => Algo::LeastDataEviction,
            s if s.to_lowercase() == "least_factorial" => Algo::LeastFactorial,
            s if s.to_lowercase() == "levenshtein_distance" => Algo::LevenShteinDistance,
            s if s.to_lowercase() == "linked_list" => Algo::LinkedList,
            s if s.to_lowercase() == "max_sibling_product" => Algo::MaxSiblingProduct,
            s if s.to_lowercase() == "max_subset_sum" => Algo::MaxSubSetSum,
            s if s.to_lowercase() == "max_with_lessdigit" => Algo::MaxWithLessDigit,
            s if s.to_lowercase() == "min_breakdown_sum" => Algo::MinBreakdownSum,
            s if s.to_lowercase() == "min_reward" => Algo::MinReward,
            s if s.to_lowercase() == "minmax_stack" => Algo::MinMaxStack,
            s if s.to_lowercase() == "number_grouping" => Algo::NumberGrouping,
            s if s.to_lowercase() == "opposite_loc_in_circle" => Algo::OppositePosInCircle,
            s if s.to_lowercase() == "pascal_triangle" => Algo::PascalTriangle,
            s if s.to_lowercase() == "permutation" => Algo::Permutation,
            s if s.to_lowercase() == "preorder_traversal" => Algo::PreOrderTraversal,
            s if s.to_lowercase() == "product_array_sort" => Algo::ProductArraySort,
            s if s.to_lowercase() == "radix_sort" => Algo::RadixSort,
            s if s.to_lowercase() == "random_perm" => Algo::RandomPerm,
            s if s.to_lowercase() == "ranking" => Algo::Ranking,
            s if s.to_lowercase() == "reconstruct_bst_from_preorder" => Algo::ReconstructBSTFromPreorder,
            s if s.to_lowercase() == "repeatproduct" => Algo::RepeatProduct,
            s if s.to_lowercase() == "request_per_sec" => Algo::RequestPerSec,
            s if s.to_lowercase() == "reverse_poland" => Algo::ReversePoland,
            s if s.to_lowercase() == "reverse_words" => Algo::ReverseWords,
            s if s.to_lowercase() == "rightmost_diffbit" => Algo::RightMostDiffBit,
            s if s.to_lowercase() == "rightmost_samebit" => Algo::RightMostSameBit,
            s if s.to_lowercase() == "runlength" => Algo::RunLength,
            s if s.to_lowercase() == "shapearea" => Algo::ShapeArea,
            s if s.to_lowercase() == "shorten_path" => Algo::ShortenPath,
            s if s.to_lowercase() == "sigma_k" => Algo::SigmaK,
            s if s.to_lowercase() == "smallest_positive_product" => Algo::SmallestPositiveProduct,
            s if s.to_lowercase() == "strange_bank" => Algo::StrangeBank,
            s if s.to_lowercase() == "string_construction" => Algo::StringConstruction,
            s if s.to_lowercase() == "string_pattern" => Algo::StringPattern,
            s if s.to_lowercase() == "sum_of_arithmatic_progression" => Algo::SumOfArithmaticProgression,
            s if s.to_lowercase() == "swap_sibling" => Algo::SwapSibling,
            s if s.to_lowercase() == "tandemrepeat" => Algo::TandemRepeat,
            s if s.to_lowercase() == "three_number_sort" => Algo::ThreeNumberSort,
            s if s.to_lowercase() == "three_sum" => Algo::ThreeSum,
            s if s.to_lowercase() == "toggle_bit" => Algo::ToggleBit,
            s if s.to_lowercase() == "turn_commands" => Algo::TurnCommands,
            s if s.to_lowercase() == "twosum" => Algo::TwoSum,
            s if s.to_lowercase() == "selectionsort" => Algo::SelectionSort,
            s if s.to_lowercase() == "waterarea" => Algo::WaterArea,
            _ => panic!("{} has not implemented yet", algo_str),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub algo: String,
}
