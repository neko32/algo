pub mod two_sum;
pub mod selection_sort;
use clap::Parser;

pub mod runner {

    use super::*;
    use two_sum::two_sum;
    use selection_sort::selection_sort;
    
    pub fn exec(algo: Algo) {
        match algo {
            Algo::TwoSum => {
                two_sum::run();
            },
            Algo::SelectionSort => {
                selection_sort::run();
            }
            _ => {
                panic!("not supported");
            }
        }    
    }   
}

#[cfg(test)]
mod test_runner {
    use crate::two_sum::two_sum;
    use crate::selection_sort::selection_sort;
    use std::collections::HashSet;

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
    TwoSum,
    SelectionSort,
}

impl Algo {
    pub fn from_str(algo_str:&str) -> Self {
        match algo_str {
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