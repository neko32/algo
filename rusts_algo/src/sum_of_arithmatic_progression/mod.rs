
pub fn exec(sum_n:u32, n:u32, h:i32, d:i32) -> i32 {
    assert!(sum_n <= n);
    let mut i = 0;
    let a = std::iter::repeat_with(||{
        i += 1;
        an(i, h, d)
    });
    let v:Vec<i32> = a.take(n as usize).collect();
    let sumv = sum_n as i32 * (v[0] + v[sum_n as usize - 1]) / 2;
    println!("progression - {:?}, sum of A{} - {}", &v, sum_n, sumv);
    sumv
}

fn an(n:u32, h:i32, d:i32) -> i32 {
    h + (n as i32 - 1) * d
}

pub fn run() {
    exec(5, 5, 4, 3);
}