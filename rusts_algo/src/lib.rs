pub mod appear_twice;
pub mod applebox;
pub mod bintodec;
pub mod caesar_crypt;
pub mod camelcase;
pub mod century;
pub mod clean_kth_bit;
pub mod cyclic_chars;
pub mod decode_reverse_poland;
pub mod dectobin;
pub mod diagonal;
pub mod euclidean;
pub mod fizzbuzz;
pub mod is_mac_addr;
pub mod kadane;
pub mod lcs;
pub mod max_sibling_product;
pub mod product_array_sort;
pub mod repeat_product;
pub mod reverse_poland_calc;
pub mod rightmost_diffbit;
pub mod rightmost_samebit;
pub mod shared;
pub mod selection_sort;
pub mod smallest_positive_product;
pub mod string_construction;
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
    use bintodec::bin_to_dec;
    use caesar_crypt::caesar_crypt;
    use camelcase::camelcase;
    use century::century;
    use clean_kth_bit::clean_kth_bit;
    use cyclic_chars::cyclic_chars;
    use decode_reverse_poland::decode_reverse_poland;
    use dectobin::dectobin;
    use diagonal::diagonal;
    use euclidean::euclidean;
    use fizzbuzz::fizzbuzz;
    use is_mac_addr::is_mac_addr;
    use kadane::kadane;
    use lcs::lcs;
    use max_sibling_product::max_sibling_product;
    use product_array_sort::product_array_sort;
    use repeat_product::repeat_product;
    use reverse_poland_calc::reverse_poland_calc;
    use rightmost_diffbit::rightmost_diffbit;
    use rightmost_samebit::rightmost_samebit;
    use selection_sort::selection_sort;
    use smallest_positive_product::smallest_positive_product;
    use string_construction::string_construction;
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
            Algo::Euclidean => {
                euclidean::run();
            }
            Algo::FizzBuzz => {
                fizzbuzz::run();
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
            Algo::MaxSiblingProduct => {
                max_sibling_product::run();
            }
            Algo::ProductArraySort => {
                product_array_sort::run();
            }
            Algo::RepeatProduct => {
                repeat_product::run();
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
            Algo::SelectionSort => {
                selection_sort::run();
            }
            Algo::SmallestPositiveProduct => {
                smallest_positive_product::run();
            }
            Algo::StringConstruction => {
                string_construction::run();
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
    use crate::bintodec::bin_to_dec;
    use crate::caesar_crypt::caesar_crypt;
    use crate::camelcase::camelcase;
    use crate::century::century;
    use crate::clean_kth_bit::clean_kth_bit;
    use crate::cyclic_chars::cyclic_chars;
    use crate::decode_reverse_poland::decode_reverse_poland;
    use crate::dectobin::dectobin;
    use crate::diagonal::diagonal;
    use crate::euclidean::euclidean;
    use crate::fizzbuzz::fizzbuzz;
    use crate::is_mac_addr::is_mac_addr;
    use crate::kadane::kadane;
    use crate::lcs::lcs;
    use crate::max_sibling_product::max_sibling_product;
    use crate::product_array_sort::product_array_sort;
    use crate::repeat_product::repeat_product;
    use crate::rightmost_diffbit::rightmost_diffbit;
    use crate::rightmost_samebit::rightmost_samebit;
    use crate::reverse_poland_calc::reverse_poland_calc;
    use crate::selection_sort::selection_sort;
    use crate::smallest_positive_product::smallest_positive_product;
    use crate::shared::shared::*;
    use crate::string_construction::string_construction;
    use crate::swap_sibling::swap_sibling;
    use crate::tandem_repeat::tandem_repeat;
    use crate::toggle_bit::toggle_bit;
    use crate::two_sum::two_sum;
    use crate::waterarea::waterarea;
    use std::collections::{BTreeSet, HashSet};
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
    fn is_mac_addr_good() {
        let mac = "00-1B-63-84-45-E6";
        assert_eq!(is_mac_addr::exec(mac), true);
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
    fn max_sibling_product_test () {
        let s = vec![3, 6, -2, -5, 7, 3];
        let r = max_sibling_product::exec(s);
        assert_eq!(r, 21);
    }

    #[test]
    fn lcs_test2() {
        let m = "pirikapirirara";
        let n = "poporinapeperuto";
        assert_eq!(lcs::exec(m, n), 6);
    }

    #[test]
    fn repeat_product_test() {
        let n = 16;
        assert_eq!(repeat_product::exec(n), 9);
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
    BinToDec,
    CamelCase,
    Century,
    CaesarCrypt,
    CleanKthBit,
    CyclicChars,
    DecodeReversePoland,
    DecToBin,
    Diagonal,
    Euclidean,
    FizzBuzz,
    IsMacAddr,
    Kadane,
    LCS,
    MaxSiblingProduct,
    ProductArraySort,
    RepeatProduct,
    ReversePoland,
    RightMostDiffBit,
    RightMostSameBit,
    SelectionSort,
    SmallestPositiveProduct,
    SwapSibling,
    StringConstruction,
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
            s if s.to_lowercase() == "euclidean" => {
                Algo::Euclidean
            }
            s if s.to_lowercase() == "fizzbuzz" => {
                Algo::FizzBuzz
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
            s if s.to_lowercase() == "max_sibling_product" => {
                Algo::MaxSiblingProduct
            }
            s if s.to_lowercase() == "product_array_sort" => {
                Algo::ProductArraySort
            }
            s if s.to_lowercase() == "repeatproduct" => {
                Algo::RepeatProduct
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
            s if s.to_lowercase() == "smallest_positive_product" => {
                Algo::SmallestPositiveProduct
            }
            s if s.to_lowercase() == "swap_sibling" => {
                Algo::SwapSibling
            }
            s if s.to_lowercase() == "string_construction" => {
                Algo::StringConstruction
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