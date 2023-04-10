
pub fn exec(v:&[i32]) -> bool {
    let len = v.len();
    if len <= 2 {
        println!("length {len} doesn't satisfy precondition");
        return false;
    }
    let mid_v = if len % 2 == 1 {
        v[len / 2]
    } else {
        v[(len / 2) - 1] + v[(len / 2)]
    };
    let rez = v[0] == mid_v && v[len - 1] == mid_v;
    println!("input - {:?}, is smooth ? {rez}", v);
    rez
}

pub fn run() {
    exec(&[7, 2, 2, 5, 10, 7]);
    exec(&[-5, -5, 10]);
    exec(&[7, 3, 7, 3, 7]);
    exec(&[7, 2, 2, 5, 10, 2]);
    exec(&[7, 3, 7, 3, 5]);
    exec(&[3, 9, 3]);
    exec(&[2, 4]);
    exec(&[4, 4]);
    exec(&[5]);
}
