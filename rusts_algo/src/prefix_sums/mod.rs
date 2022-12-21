
pub fn exec(a:&[i32]) -> Vec<i32> {
    let rez:Vec<i32> = a.iter().fold(Vec::new(), |mut acc, n|{
        acc.push(acc.last().unwrap_or(&0) + n);
        acc
    });
    println!("input - {:?}, result - {:?}", a, rez);
    rez
}

pub fn run() {
    let v = [1, 2, 3];
    exec(&v);
}