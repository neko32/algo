

pub fn exec(v:&[i32]) -> i32 {
    let len = v.len();
    let r = v[((len - 1) / 2) as usize];
    println!("input - {:?}, answer - {r}", v);
    r
}

pub fn run() {
    exec(&[2, 4, 7]);
    exec(&[2, 3]);
}
