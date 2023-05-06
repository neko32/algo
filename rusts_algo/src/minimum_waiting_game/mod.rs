
pub fn exec(w:&[i32]) -> i32 {
    let mut v:Vec<i32> = Vec::new();
    v.extend_from_slice(w);
    v.sort();
    v.pop();
    let mut cumsum = 0;
    for i in 0..v.len() {
        cumsum = cumsum + v[i];
    }
    println!("input - {:?}, result - {cumsum}", w);
    cumsum
}

pub fn run() {
    exec(&[5, 1, 4]);
}
