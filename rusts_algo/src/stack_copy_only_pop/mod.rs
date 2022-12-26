
pub fn exec(src:&mut Vec<i32>, dest:&mut Vec<i32>) {
    match src.pop() {
        None => (),
        Some(v) => {
            exec(src, dest);
            dest.push(v);
        }
    }
}

pub fn run() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut r:Vec<i32> = Vec::new();
    exec(&mut v, &mut r);
    println!("{:?}", r);
}
