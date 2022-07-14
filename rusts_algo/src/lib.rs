pub mod appear_twice;
pub mod bintodec;
pub mod caesar_crypt;
pub mod camelcase;
pub mod dectobin;
pub mod kadane;
pub mod repeat_product;
pub mod rightmost_diffbit;
pub mod rightmost_samebit;
pub mod selection_sort;
pub mod smallest_positive_product;
pub mod tandem_repeat;
pub mod toggle_bit;
pub mod two_sum;

use clap::Parser;

pub mod runner {

    use super::*;
    use appear_twice::appear_twice;
    use bintodec::bin_to_dec;
    use caesar_crypt::caesar_crypt;
    use camelcase::camelcase;
    use dectobin::dectobin;
    use kadane::kadane;
    use repeat_product::repeat_product;
    use rightmost_diffbit::rightmost_diffbit;
    use rightmost_samebit::rightmost_samebit;
    use selection_sort::selection_sort;
    use smallest_positive_product::smallest_positive_product;
    use tandem_repeat::tandem_repeat;
    use toggle_bit::toggle_bit;
    use two_sum::two_sum;
    
    pub fn exec(algo: Algo) {
        match algo {
            Algo::AppearTwice => {
                appear_twice::run();
            },
            Algo::BinToDec => {
                bin_to_dec::run();
            },
            Algo::CaesarCrypt => {
                caesar_crypt::run();
            }
            Algo::CamelCase => {
                camelcase::run();
            },
            Algo::DecToBin => {
                dectobin::run();
            },
            Algo::Kadane => {
                kadane::run();
            },
            Algo::RepeatProduct => {
                repeat_product::run();
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
            Algo::TandemRepeat => {
                tandem_repeat::run()
            },
            Algo::ToggleBit => {
                toggle_bit::run();
            }
            Algo::TwoSum => {
                two_sum::run();
            },
        }    
    }   
}

#[cfg(test)]
mod test_runner {
    use crate::appear_twice::appear_twice;
    use crate::bintodec::bin_to_dec;
    use crate::caesar_crypt::caesar_crypt;
    use crate::camelcase::camelcase;
    use crate::dectobin::dectobin;
    use crate::kadane::kadane;
    use crate::repeat_product::repeat_product;
    use crate::rightmost_diffbit::rightmost_diffbit;
    use crate::rightmost_samebit::rightmost_samebit;
    use crate::selection_sort::selection_sort;
    use crate::smallest_positive_product::smallest_positive_product;
    use crate::tandem_repeat::tandem_repeat;
    use crate::toggle_bit::toggle_bit;
    use crate::two_sum::two_sum;
    use std::collections::{BTreeSet, HashSet};

    #[test]
    fn appear_twice_test() {
        let expected:BTreeSet<char> = BTreeSet::from_iter(['b', 'd']);
        let rez = appear_twice::exec();
        assert_eq!(expected, rez);
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
    fn kadane_test() {
        let v = vec![3,5,-9,1,3,-2,3,4,7,2,-9,6,3,1,-5,4];
        assert_eq!(19, kadane::exec(v));
    }

    #[test]
    fn repeat_product_test() {
        let n = 16;
        assert_eq!(repeat_product::exec(n), 9);
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
}


pub enum Algo {
    AppearTwice,
    BinToDec,
    CaesarCrypt,
    CamelCase,
    DecToBin,
    Kadane,
    RepeatProduct,
    RightMostDiffBit,
    RightMostSameBit,
    SelectionSort,
    SmallestPositiveProduct,
    TandemRepeat,
    ToggleBit,
    TwoSum,
}

impl Algo {
    pub fn from_str(algo_str:&str) -> Self {
        match algo_str {
            s if s.to_lowercase() == "appear_twice" => {
                Algo::AppearTwice
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
            s if s.to_lowercase() == "dectobin" => {
                Algo::DecToBin
            }
            s if s.to_lowercase() == "kadane" => {
                Algo::Kadane
            }
            s if s.to_lowercase() == "repeatproduct" => {
                Algo::RepeatProduct
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