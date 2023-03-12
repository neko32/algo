

pub fn exec(legs:i32) -> Vec<u32> {
    let mut n_cats = legs / 4;
    let mut v:Vec<u32> = Vec::new();
    while n_cats > 0 {
        v.push(((legs - (n_cats * 4)) / 2) as u32);
        n_cats -= 1;
    }
    v.push((legs / 2) as u32);
    println!("legs - {legs} and result - {:?}", v);
    v
}

pub fn run() {
    exec(6);
    exec(2);
}
