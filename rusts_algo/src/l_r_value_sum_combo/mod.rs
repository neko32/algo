
pub fn exec(n:i32, a:i32, b:i32) -> i32 {

    let mut l = a.min(b);
    let mut r = a.max(b);
    let mut cnt = 0;

    while l <= r {
        let s = l + r;
        if s == n {
            println!("hit - {l}+{r}={n}");
            cnt += 1;
            l += 1;
            r -= 1;
        } else if s < n {
            l += 1;
        } else {
            r -= 1;
        }
    }

    println!("n={n},a={a},b={b},result={cnt}");
    cnt
}

pub fn run() {
    exec(6, 4, 2);
    exec(6, 2, 4);
    exec(7, 2, 4);
}
