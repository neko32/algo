
pub fn exec(a:&[i32]) -> Vec<i32> {

    let maxv:i32 = *a.iter().max().unwrap();
    let mut b:Vec<i32> = Vec::with_capacity((maxv + 1) as usize);
    (0..=maxv).into_iter().for_each(|_|b.push(0));

    for v in a {
        b[*v as usize] += 1;
    }

    println!("input - {:?}, bincount - {:?}", a, b);
    b
}

pub fn run() {
    let a = [7, 3, 1, 10, 5, 0, 0, 1, 2, 1, 8, 1, 9, 1, 7];
    exec(&a);
}
