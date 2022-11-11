
use std::cmp::max;

pub fn exec(a:&[i32]) -> i32 {
    let len = a.len();
    let mut s:Vec<i32> = Vec::from_iter(std::iter::repeat(0).take(len));

    if len == 0 {
        return -1;
    }

    if len == 1 {
        return a[0];
    }

    s[0] = a[0];
    s[1] = max(a[0], a[1]);

    for i in 2..len {
        s[i] = max(s[i - 2] + a[i], s[i - 1]);
    }

    let rez = *s.last().unwrap_or(&-1);

    println!("{:?}, result is {}", s, rez);

    rez

}

pub fn run() {
    let a = [75, 105, 120, 75, 90, 135];
    exec(&a);
}

