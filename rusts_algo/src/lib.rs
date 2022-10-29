pub mod appear_twice;
pub mod applebox;
pub mod array_product_sum;
pub mod bintodec;
pub mod caesar_crypt;
pub mod camelcase;
pub mod century;
pub mod clean_kth_bit;
pub mod cyclic_chars;
pub mod decode_reverse_poland;
pub mod dectobin;
pub mod diagonal;
pub mod document_build;
pub mod even_num_sum;
pub mod euclidean;
pub mod fizzbuzz;
pub mod helon_formula;
pub mod is_palindrome;
pub mod is_mac_addr;
pub mod kadane;
pub mod lcs;
pub mod least_factorial;
pub mod max_sibling_product;
pub mod max_with_lessdigit;
pub mod preorder_traversal;
pub mod product_array_sort;
pub mod radix_sort;
pub mod repeat_product;
pub mod request_per_sec;
pub mod reverse_poland_calc;
pub mod rightmost_diffbit;
pub mod rightmost_samebit;
pub mod runlength;
pub mod shared;
pub mod selection_sort;
pub mod smallest_positive_product;
pub mod string_construction;
pub mod string_pattern;
pub mod swap_sibling;
pub mod tandem_repeat;
pub mod toggle_bit;
pub mod two_sum;
pub mod waterarea;

use clap::Parser;

pub mod runner {

    use super::*;
    use appear_twice::appear_twice;
    use applebox::applebox;
    use array_product_sum::array_product_sum;
    use bintodec::bin_to_dec;
    use caesar_crypt::caesar_crypt;
    use camelcase::camelcase;
    use century::century;
    use clean_kth_bit::clean_kth_bit;
    use cyclic_chars::cyclic_chars;
    use decode_reverse_poland::decode_reverse_poland;
    use dectobin::dectobin;
    use diagonal::diagonal;
    use document_build::document_build;
    use even_num_sum::even_num_sum;
    use euclidean::euclidean;
    use helon_formula::helon_formula;
    use fizzbuzz::fizzbuzz;
    use is_palindrome::is_palindrome;
    use is_mac_addr::is_mac_addr;
    use kadane::kadane;
    use lcs::lcs;
    use least_factorial::least_factorial;
    use max_sibling_product::max_sibling_product;
    use max_with_lessdigit::max_with_lessdigit;
    use preorder_traversal::preorder_traversal;
    use product_array_sort::product_array_sort;
    use radix_sort::radix_sort;
    use repeat_product::repeat_product;
    use request_per_sec::request_per_sec;
    use reverse_poland_calc::reverse_poland_calc;
    use rightmost_diffbit::rightmost_diffbit;
    use rightmost_samebit::rightmost_samebit;
    use runlength::runlength;
    use selection_sort::selection_sort;
    use smallest_positive_product::smallest_positive_product;
    use string_construction::string_construction;
    use string_pattern::string_pattern;
    use swap_sibling::swap_sibling;
    use tandem_repeat::tandem_repeat;
    use toggle_bit::toggle_bit;
    use two_sum::two_sum;
    use waterarea::waterarea;
    
    pub fn exec(algo: Algo) {
        match algo {
            Algo::AppearTwice => {
                appear_twice::run();
            },
            Algo::Applebox => {
                applebox::run();
            }
            Algo::ArrayProductSum => {
                array_product_sum::run();
            }
            Algo::BinToDec => {
                bin_to_dec::run();
            },
            Algo::CaesarCrypt => {
                caesar_crypt::run();
            }
            Algo::CamelCase => {
                camelcase::run();
            },
            Algo::Century => {
                century::run();
            }
            Algo::CleanKthBit => {
                clean_kth_bit::run();
            }
            Algo::CyclicChars => {
                cyclic_chars::run();
            }
            Algo::DecodeReversePoland => {
                decode_reverse_poland::run();
            }
            Algo::DecToBin => {
                dectobin::run();
            },
            Algo::Diagonal => {
                diagonal::run();
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
            Algo::FizzBuzz => {
                fizzbuzz::run();
            }
            Algo::HelonFormula => {
                helon_formula::run();
            }
            Algo::IsPalindrome => {
                is_palindrome::run();
            }
            Algo::IsMacAddr => {
                is_mac_addr::run();
            }
            Algo::Kadane => {
                kadane::run();
            },
            Algo::LCS => {
                lcs::run();
            },
            Algo::RadixSort => {
                radix_sort::run();
            }
            Algo::LeastFactorial => {
                least_factorial::run();
            }
            Algo::MaxSiblingProduct => {
                max_sibling_product::run();
            }
            Algo::MaxWithLessDigit => {
                max_with_lessdigit::run();
            }
            Algo::PreOrderTraversal => {
                preorder_traversal::run();
            }
            Algo::ProductArraySort => {
                product_array_sort::run();
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
            Algo::RightMostDiffBit => {
                rightmost_diffbit::run();
            }
            Algo::RightMostSameBit => {
                rightmost_samebit::run();
            },
            Algo::RunLength => {
                runlength::run();
            }
            Algo::SelectionSort => {
                selection_sort::run();
            }
            Algo::SmallestPositiveProduct => {
                smallest_positive_product::run();
            }
            Algo::StringConstruction => {
                string_construction::run();
            }
            Algo::StringPattern => {
                string_pattern::run();
            }
            Algo::SwapSibling => {
                swap_sibling::run();
            }
            Algo::TandemRepeat => {
                tandem_repeat::run()
            },
            Algo::ToggleBit => {
                toggle_bit::run();
            }
            Algo::TwoSum => {
                two_sum::run();
            },
            Algo::WaterArea => {
                waterarea::run();
            }
        }    
    }   
}

#[cfg(test)]
mod test_runner {
    use crate::appear_twice::appear_twice;
    use crate::applebox::applebox;
    use crate::array_product_sum::array_product_sum;
    use crate::bintodec::bin_to_dec;
    use crate::caesar_crypt::caesar_crypt;
    use crate::camelcase::camelcase;
    use crate::century::century;
    use crate::clean_kth_bit::clean_kth_bit;
    use crate::cyclic_chars::cyclic_chars;
    use crate::decode_reverse_poland::decode_reverse_poland;
    use crate::dectobin::dectobin;
    use crate::diagonal::diagonal;
    use crate::document_build::document_build;
    use crate::even_num_sum::even_num_sum;
    use crate::euclidean::euclidean;
    use crate::fizzbuzz::fizzbuzz;
    use crate::helon_formula::helon_formula;
    use crate::is_palindrome::is_palindrome;
    use crate::is_mac_addr::is_mac_addr;
    use crate::kadane::kadane;
    use crate::lcs::lcs;
    use crate::least_factorial::least_factorial;
    use crate::max_sibling_product::max_sibling_product;
    use crate::radix_sort::radix_sort;
    use crate::max_with_lessdigit::max_with_lessdigit;
    use crate::preorder_traversal::preorder_traversal;
    use crate::request_per_sec::request_per_sec;
    use crate::product_array_sort::product_array_sort;
    use crate::repeat_product::repeat_product;
    use crate::rightmost_diffbit::rightmost_diffbit;
    use crate::rightmost_samebit::rightmost_samebit;
    use crate::runlength::runlength;
    use crate::reverse_poland_calc::reverse_poland_calc;
    use crate::selection_sort::selection_sort;
    use crate::smallest_positive_product::smallest_positive_product;
    use crate::shared::shared::*;
    use crate::string_construction::string_construction;
    use crate::string_pattern::string_pattern;
    use crate::swap_sibling::swap_sibling;
    use crate::tandem_repeat::tandem_repeat;
    use crate::toggle_bit::toggle_bit;
    use crate::two_sum::two_sum;
    use crate::waterarea::waterarea;
    use std::{collections::{BTreeSet, HashSet}, array};
    use float_cmp::approx_eq;
    use num::integer::gcd;

    #[test]
    fn appear_twice_test() {
        let expected:BTreeSet<char> = BTreeSet::from_iter(['b', 'd']);
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
    fn array_product_sum_test() {
        let n = [5, 1, 4, 2];
        let p = &n[..];
        let r = array_product_sum::exec(p);
        assert_eq!(r, vec![8, 40, 10, 20]);
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
    fn clean_kth_bit_test() {
        let n = 127;
        let k = 3;
        assert_eq!(clean_kth_bit::exec(n, k), 123);
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
    fn test_fizzbuzz() {
        let n = 30;
        let rez = fizzbuzz::exec(n);
        let expected:String = "12fizz4buzzfizz78fizzbuzz11fizz1314fizzbuzz1617fizz19buzzfizz2223fizzbuzz26fizz2829fizzbuzz".to_string();
        assert_eq!(rez, expected);
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
        let v = vec![3,5,-9,1,3,-2,3,4,7,2,-9,6,3,1,-5,4];
        assert_eq!(19, kadane::exec(v));
    }

    #[test]
    fn lcs_test1() {
        let m = "abcde";
        let n = "acbef";
        assert_eq!(lcs::exec(m, n), 3);
    }

    #[test]
    fn least_factorial_test() {
        let n = 17;
        assert_eq!(least_factorial::exec(n), 24);
    }

    #[test]
    fn max_sibling_product_test () {
        let s = vec![3, 6, -2, -5, 7, 3];
        let r = max_sibling_product::exec(s);
        assert_eq!(r, 21);
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
    fn preorder_trav_test() {
        let v:Vec<i32> = vec![5, 9, 2, 10, 1, 4];
        let r:Vec<i32> = preorder_traversal::exec(v);
        assert_eq!(r, vec![5, 2, 1, 4, 9, 10]);
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
        assert_eq!(rez, 928);
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
    fn diagonal_is_diag_yes() {
        let a = Point::new(1, 5);
        let b = Point::new(9, 13);
        println!("a:{},b:{} must be diagonal - result is {}", a, b, diagonal::exec(&a, &b));
        assert!(diagonal::exec(&a, &b));
    }


    #[test]
    fn diagonal_is_diag_no() {
        let a = Point::new(3, 5);
        let b = Point::new(8, 5);
        println!("a:{},b:{} must be diagonal - result is {}", a, b, diagonal::exec(&a, &b));
        assert!(diagonal::exec(&a, &b) == false);
    }

    #[test]
    fn document_build_test() {
        let s = "Todd told Tom to trot to the timber".to_string();
        let expected:Vec<char> = vec![' ', 'T', 'b', 'd', 'e', 'h', 'i', 'l', 'm', 'o', 'r', 't'];
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
        let mut v:Vec<i32> = (1..=6).collect();
        swap_sibling::exec(&mut v);
        let expected:Vec<i32> = vec![2, 1, 4, 3, 6, 5];
        assert_eq!(v, expected);
    }

    #[test]
    fn tandem_repeat_case2() {
        let a = "ABA".to_string();
        let b = "cattac".to_string();
        assert!(!tandem_repeat::exec(a) && !tandem_repeat::exec(b));
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
    fn two_sum_case1() {
        let rez = two_sum::exec(vec![3,5,-4,8,11,1,-1,6], 10);
        assert_eq!(rez, vec![-1,11])
    }

    #[test]
    fn two_sum_case2() {
        let rez = two_sum::exec(vec![4, 6], 10);
        let rset:HashSet<i32> = HashSet::from_iter(rez); 
        let expected:HashSet<i32> = HashSet::from_iter([4, 6]);
        assert_eq!(rset, expected);
    }

    #[test]
    fn two_sum_case3() {
        let rez = two_sum::exec(vec![-21, 301, 12, 4, 65, 56, 210, 356, 9, -47], 163);
        let rset:HashSet<i32> = HashSet::from_iter(rez); 
        let expected:HashSet<i32> = HashSet::from_iter([210, -47]);
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
        let a:Vec<u32> = vec![0, 8, 0, 0, 5, 0, 0, 10, 0, 0, 1, 1, 0, 3];
        let r = waterarea::exec(a);
        assert_eq!(r, 48);
    }
}


pub enum Algo {
    AppearTwice,
    Applebox,
    ArrayProductSum,
    BinToDec,
    CamelCase,
    Century,
    CaesarCrypt,
    CleanKthBit,
    CyclicChars,
    DecodeReversePoland,
    DecToBin,
    Diagonal,
    DocumentBuild,
    EvenNumSum,
    Euclidean,
    FizzBuzz,
    HelonFormula,
    IsMacAddr,
    IsPalindrome,
    Kadane,
    LCS,
    LeastFactorial,
    MaxSiblingProduct,
    MaxWithLessDigit,
    PreOrderTraversal,
    ProductArraySort,
    RadixSort,
    RepeatProduct,
    RequestPerSec,
    ReversePoland,
    RightMostDiffBit,
    RightMostSameBit,
    RunLength,
    SelectionSort,
    SmallestPositiveProduct,
    SwapSibling,
    StringConstruction,
    StringPattern,
    TandemRepeat,
    ToggleBit,
    TwoSum,
    WaterArea,
}

impl Algo {
    pub fn from_str(algo_str:&str) -> Self {
        match algo_str {
            s if s.to_lowercase() == "appear_twice" => {
                Algo::AppearTwice
            }
            s if s.to_lowercase() == "applebox" => {
                Algo::Applebox
            }
            s if s.to_lowercase() == "array_product_sum" => {
                Algo::ArrayProductSum
            }
            s if s.to_lowercase() == "bintodec" => {
                Algo::BinToDec
            }
            s if s.to_lowercase() == "caesar_crypt" => {
                Algo::CaesarCrypt
            }
            s if s.to_lowercase() == "camelcase" => {
                Algo::CamelCase
            }
            s if s.to_lowercase() == "century" => {
                Algo::Century
            }
            s if s.to_lowercase() == "clean_kth_bit" => {
                Algo::CleanKthBit
            }
            s if s.to_lowercase() == "cyclic_chars" => {
                Algo::CyclicChars
            }
            s if s.to_lowercase() == "decode_reverse_poland" => {
                Algo::DecodeReversePoland
            }
            s if s.to_lowercase() == "dectobin" => {
                Algo::DecToBin
            }
            s if s.to_lowercase() == "diagonal" => {
                Algo::Diagonal
            }
            s if s.to_lowercase() == "document_build" => {
                Algo::DocumentBuild
            }
            s if s.to_lowercase() == "even_num_sum" => {
                Algo::EvenNumSum
            }
            s if s.to_lowercase() == "euclidean" => {
                Algo::Euclidean
            }
            s if s.to_lowercase() == "fizzbuzz" => {
                Algo::FizzBuzz
            }
            s if s.to_lowercase() == "helon_formula" => {
                Algo::HelonFormula
            }
            s if s.to_lowercase() == "is_palindrome" => {
                Algo::IsPalindrome
            }
            s if s.to_lowercase() == "is_mac_addr" => {
                Algo::IsMacAddr
            }
            s if s.to_lowercase() == "kadane" => {
                Algo::Kadane
            }
            s if s.to_lowercase() == "lcs" => {
                Algo::LCS
            }
            s if s.to_lowercase() == "least_factorial" => {
                Algo::LeastFactorial
            }
            s if s.to_lowercase() == "max_sibling_product" => {
                Algo::MaxSiblingProduct
            }
            s if s.to_lowercase() == "max_with_lessdigit" => {
                Algo::MaxWithLessDigit
            }
            s if s.to_lowercase() == "preorder_traversal" => {
                Algo::PreOrderTraversal
            }
            s if s.to_lowercase() == "product_array_sort" => {
                Algo::ProductArraySort
            }
            s if s.to_lowercase() == "radix_sort" => {
                Algo::RadixSort
            }
            s if s.to_lowercase() == "repeatproduct" => {
                Algo::RepeatProduct
            }
            s if s.to_lowercase() == "request_per_sec" => {
                Algo::RequestPerSec
            }
            s if s.to_lowercase() == "reverse_poland" => {
                Algo::ReversePoland
            }
            s if s.to_lowercase() == "rightmost_diffbit" => {
                Algo::RightMostDiffBit
            }
            s if s.to_lowercase() == "rightmost_samebit" => {
                Algo::RightMostSameBit
            }
            s if s.to_lowercase() == "runlength" => {
                Algo::RunLength
            }
            s if s.to_lowercase() == "smallest_positive_product" => {
                Algo::SmallestPositiveProduct
            }
            s if s.to_lowercase() == "swap_sibling" => {
                Algo::SwapSibling
            }
            s if s.to_lowercase() == "string_construction" => {
                Algo::StringConstruction
            }
            s if s.to_lowercase() == "string_pattern" => {
                Algo::StringPattern
            }
            s if s.to_lowercase() == "tandemrepeat" => {
                Algo::TandemRepeat
            }
            s if s.to_lowercase() == "toggle_bit" => {
                Algo::ToggleBit
            }
            s if s.to_lowercase() == "twosum" => {
                Algo::TwoSum 
            },
            s if s.to_lowercase() == "selectionsort" => {
                Algo::SelectionSort
            }
            s if s.to_lowercase() == "waterarea" => {
                Algo::WaterArea
            }
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