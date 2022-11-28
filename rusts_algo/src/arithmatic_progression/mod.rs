

pub fn exec(n:u32, h:u32, d:u32) -> u32 {
    let nv = h + (n - 1) * d;
    println!("N{} = {} + ({} - 1) * {} = {}", n, h, n, d, nv);
    nv
}

pub fn run() {
    exec(4, 5, 2);
    exec(5, 5, 2);
}
