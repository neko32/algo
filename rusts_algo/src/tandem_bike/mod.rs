
pub fn exec(a:&mut [i32], b:&mut [i32], fastest:bool) -> i32 {
    let mut r = 0;
    println!("input - {:?} & {:?}", a, b);
    a.sort();
    if fastest {
        b.sort_by(|x,y|y.cmp(x));
    } else {
        b.sort_by(|x,y|x.cmp(y));
    }
    for i in 0..a.len() {
        r += a[i].max(b[i]);
    }
    println!("result - {r}");
    r
}

pub fn run() {
    exec(&mut [5, 5, 3, 9, 2], &mut [1, 6, 7, 2, 1], true);
    exec(&mut [5, 5, 3, 9, 2], &mut [1, 6, 7, 2, 1], false);
    // 2, 3, 5, 5, 9
    // 7, 6, 2, 1, 1 
    // 1, 1, 2, 6, 7
    // 7, 6, 5, 5, 9 <- fastest
    // 2, 3, 5, 6, 9 <- slowest
}
