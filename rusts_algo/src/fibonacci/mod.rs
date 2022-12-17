
use std::collections::HashMap;
pub fn exec(n:u32) -> u64 {
    let mut h:HashMap<u32, u64> = HashMap::new();
    fib(n, &mut h)
}

fn fib(n:u32, memo:&mut HashMap<u32, u64>) -> u64 {
    match n {
        0 | 1 | 2 => 1,
        _ => {
            let r1 = match memo.get(&(n - 1)) {
                Some(v) => *v,
                None => {
                    let r = fib(n - 1, memo);
                    memo.insert(n - 1, r);
                    r
                },
            };
            let r2 = match memo.get(&(n - 2)) {
                Some(v) => *v,
                None => {
                    let r = fib(n - 2, memo);
                    memo.insert(n - 2, r);
                    r
                },
            };
            r1 + r2
        },
    }
}

pub fn run() {
    let n = 8;
    println!("n = {}, fib = {}", n, exec(n));
}
