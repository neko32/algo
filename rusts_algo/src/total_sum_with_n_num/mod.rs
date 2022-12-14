

use ndarray::{Array2};
use std::cmp::min;
pub fn exec(a:&[i32], target:i32, k:u32) -> bool {

    let len = a.len();

    let mut dp:Array2<i32> = Array2::from_elem((len + 1, (target + 1) as usize), i32::MAX - 10000);
    dp[(0, 0)] = 0;

    for i in 0..len {
        for t in 0..=target as usize {
            if a[i] <= t as i32 {
                dp[(i + 1, t)] = min(dp[(i, t - a[i] as usize)] + 1, dp[(i, t)]);
            } else {
                dp[(i + 1, t)] = dp[(i, t)];
            }
        }
    }

    println!("done. view - {}", dp.view());
    dp[(len, target as usize)] <= k as i32
}

pub fn run() {
    let a = &[2, 3, 4, 5];
    let target = 14;
    let k = 4;
    println!("{}", exec(a, target, k));
}
