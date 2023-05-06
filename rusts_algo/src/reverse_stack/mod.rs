
pub fn exec(s:&mut Vec<i32>, buf:&mut Vec<i32>) {
    if let Some(v) = s.pop() {
        buf.push(v);
        exec(s, buf);
    }
}

pub fn run() {
    let mut buf:Vec<i32> = Vec::new();
    let mut s:Vec<i32> = Vec::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    exec(&mut s, &mut buf);
    println!("{:?}", buf);
}
