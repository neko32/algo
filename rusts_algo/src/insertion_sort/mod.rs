
pub fn exec(v:&mut [i32]) {
    let len = v.len();
    println!("before sort - {:?}", v);
    for i in 1..len {
        for j in (1..=i).rev() {
            if v[j - 1] > v[j] {
                v.swap(j - 1, j);
            }
        }
    }
    println!("after sort - {:?}", v);
}

pub fn run() {
    let mut v = [9, 15, 2, 7, 4, -5, 9, -3, 10, 8];
    exec(&mut v);
}
