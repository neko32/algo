
pub fn exec(n:i32, x:i32) -> bool {
    let r = n % x;
    println!("{n} % {x} => {r}");
    r == 0
}

pub fn run() {
    exec(10, 2);
    exec(10, 7);
    exec(1250, 5);
    exec(793, 3);
}
