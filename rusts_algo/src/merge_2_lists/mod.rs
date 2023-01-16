
pub fn exec(a:&mut [i32], b:&mut [i32]) -> Vec<i32> {
    a.sort();
    b.sort();
    let mut c:Vec<i32> = Vec::new();
    let mut a_idx = 0;
    let mut b_idx = 0;
    let a_len = a.len();
    let b_len = b.len();
    while a_idx < a_len && b_idx < b_len {
        if a[a_idx] < b[b_idx] {
            c.push(a[a_idx]);
            a_idx += 1;
        } else {
            c.push(b[b_idx]);
            b_idx += 1;
        }
    }
    while a_idx < a_len {
        c.push(a[a_idx]);
        a_idx += 1;
    }
    while b_idx < b_len {
        c.push(b[b_idx]);
        b_idx += 1;
    }
    println!("merged {:?} and {:?}, result is {:?}", a, b, c);
    c
}

pub fn run() {
    let mut a = [7, 3, 10, 2, 10, 25, 9, 11];
    let mut b = [3, 4, 19, 21, 15];
    exec(&mut a, &mut b);
}