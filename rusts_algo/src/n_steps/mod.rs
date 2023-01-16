
use std::collections::HashMap;

pub fn exec(n:i32, steps:u32) -> u32 {
    let mut memo:HashMap<i32, i32> = HashMap::new();
    let rez = stepdown(n, steps as i32, &mut memo) as u32;
    println!("{n} steps by 1 .. = {steps} - {} ways", rez);
    rez
}

fn stepdown(n:i32, steps:i32, memo:&mut HashMap<i32, i32>) -> i32 {

    match n {
        0 => 1,
        n if n < 0 => 0,
        n => {
            let mut t:Vec<i32> = Vec::new();
            for s in 1..=steps {
                let rez = {
                    match memo.get(&(n - s)) {
                        Some(cached) => *cached,
                        None => {
                            let r = stepdown(n - s, steps, memo);
                            memo.entry(n - s).or_insert(r);
                            r as i32
                        }
                    }
                };
                t.push(rez);
            }
            t.into_iter().sum()
        }
    }
}

pub fn run() {
    exec(4, 2);
}