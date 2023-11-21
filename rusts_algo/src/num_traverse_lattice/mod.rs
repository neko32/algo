
pub fn exec(r:u32, c:u32) -> u32 {
    trav(0, 0, r, c)
}

fn trav(cur_r:u32, cur_c:u32, r:u32, c:u32) -> u32 {
    if cur_r == r - 1 && cur_c == c - 1 {
        1
    } else if cur_r == r || cur_c == c {
        0
    } else {
        trav(cur_r + 1, cur_c, r, c) + trav(cur_r, cur_c + 1, r, c)
    }
}

pub fn run() {
    let r = 3;
    let c = 2;
    println!("r={},c={},result={}", r, c, exec(r, c));
}
