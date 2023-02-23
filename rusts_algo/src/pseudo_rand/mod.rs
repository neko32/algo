

pub fn exec(a:i32, c:i32, m:i32, x:i32, n:u32) -> Vec<i32> {

    // M must be some 2^k, a is number % 8 == 5, C is odd
    println!("a = {a}, c = {c}, m = {m}, x0 = {x}");
    assert!(m % 2 == 0);
    assert!(a % 8 == 5);
    assert!(c % 2 == 1);
    let mut x = x;
    let mut v:Vec<i32> = Vec::new();
    for _ in 0..n {
        x = (a * x + c) % m;
        v.push(x);
    }
    println!("v = {:?}", v);
    v
}

pub fn run() {
    exec(109, 1021, 256, 13, 20);
}

