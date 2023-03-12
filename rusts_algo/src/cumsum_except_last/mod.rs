
pub fn exec(v:&[i32]) -> i32 {
    let len = v.len();
    let rez:i32 = (&v[..len - 1]).iter().sum();
    println!("input - {:?}, result - {rez}", v);
    rez
}

pub fn run() {
    let r:Vec<i32> = (1..=5).collect();
    exec(&r[..]);
}

