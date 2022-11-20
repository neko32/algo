

use multiarray::Array1D;
use std::cmp::min;

pub fn exec(n:u32, denoms:&[u32]) -> i32 {

    const INFSIM:u32 = 10000;
    let mut dp = Array1D::new(1000, INFSIM);
    dp[0] = 0;

    for i in 0..=n {
        for d in denoms {
            dp[(i + d) as usize] =  min(dp[(i + d) as usize], dp[i as usize] + 1);
        }
    }

    let rez = dp[n as usize];
    let retv = if rez != INFSIM {
        rez as i32
    } else {
        -1
    };
    println!("n={}, denoms={:?} - result={}", n, denoms, retv);
    retv
}

pub fn run() {
    let n = 7_u32;
    let denoms = &[1, 5, 10];
    exec(n, denoms);
}
