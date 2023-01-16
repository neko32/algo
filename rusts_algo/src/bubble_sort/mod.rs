
pub fn exec(v:&mut [i32]) {
    println!("before - {:?}", v);
    for i in (1..v.len() - 1).rev() {
        for j in 0..=i {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
    println!("after - {:?}", v);
}

pub fn run() {
    let mut v = [7, 3, 10, 9, 3, 5, 11, 12, 6];
    exec(&mut v);
}
