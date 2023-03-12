
pub fn exec(v: &mut [i32]) -> i32 {
    let mut sumv = 0;
    println!("input - {:?}", v);
    v.sort();
    for x in v {
        if *x > sumv + 1 {
            println!("result - {}", sumv + 1);
            return sumv + 1
        }
        sumv += *x;
    }
    println!("result - {}", sumv + 1);
    sumv + 1
}

pub fn run() {
    exec(&mut [5, 7, 1, 1, 2, 3, 22]);
}
