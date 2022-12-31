
pub fn exec(v:i32, min_v:i32, max_v:i32) -> i32 {
    let rez = if v < min_v {
        min_v
    } else if v > max_v {
        max_v
    } else {
        v
    };
    println!("v={v},min_v={min_v},max_v={max_v} => result is {rez}");
    rez
}

pub fn run() {
    exec(72, 10, 100);
    exec(4, 10, 100);
    exec(103, 10, 100);
}