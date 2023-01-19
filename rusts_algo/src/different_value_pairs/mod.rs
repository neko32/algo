
pub fn exec(a:Vec<i32>, b:Vec<i32>) -> Vec<Vec<i32>> {
    println!("a - {:?}, b - {:?}", a, b);
    let r:Vec<_> = a.into_iter().zip(b.into_iter()).filter(|(a,b)|*a != *b).collect();
    let mut x:Vec<i32> = Vec::new();
    let mut y:Vec<i32> = Vec::new();
    for (m, n) in r {
        x.push(m);
        y.push(n);
    }
    let rez = vec![x, y];
    println!("result - {:?}", rez);
    rez
}

pub fn run() {
    let a = vec![5, 28, 14, 99, 17];
    let b = vec![5, 14, 28, 99, 16];
    exec(a, b);
}