
pub fn exec(v:&[i32]) -> u32 {
    let cnt = v.iter().filter(|x|**x != 0).count();
    println!("input - {:?}, l0 norm is {cnt}", v);
    cnt as u32
}

pub fn run() {
    let v = [-3, -1, 0, 2, 1, 5, 0, 7];
    exec(&v);
}
