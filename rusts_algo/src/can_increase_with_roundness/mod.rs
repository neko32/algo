

pub fn exec(n:i32) -> bool {
    let mut d = 0;
    let digits:i32 = n.to_string().len() as i32;
    while d < digits {
        let v = n / 10_i32.pow(d as u32) % 10;
        if v != 0 {
            break;
        }
        d += 1;
    }
    if d == digits {
        return false;
    }
    d += 1;

    while d < digits {
        let v = n / 10_i32.pow(d as u32) % 10;
        if v == 0 {
            return true;
        }
        d += 1;
    }
    false

}

pub fn run() {
    println!("{}", exec(902200100));
    println!("{}", exec(11100));
}
