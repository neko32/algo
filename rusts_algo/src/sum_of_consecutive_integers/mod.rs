
pub fn exec(n:i32) -> Vec<Vec<i32>> {
    let mut v:Vec<Vec<i32>> = Vec::new();
    for i in 1..=(n / 2) {
        let r = scan(i, n);
        if !r.is_empty() {
            v.push(r);
        }
    }
    println!("n - {n}, result - {:?}", v);
    v
}

fn scan(s:i32, n:i32) -> Vec<i32> {
    let mut v:Vec<i32> = Vec::new();
    let mut a = s;
    loop {
        let sumv = v.iter().sum::<i32>() + a;
        if sumv < n {
            v.push(a);
            a += 1;
        } else if sumv == n {
            v.push(a);
            break;
        } else {
            return Vec::new();
        }
    }
    if v.len() > 0 {
        v
    } else {
        Vec::new()
    }
}

pub fn run() {
    exec(9);
    exec(8);
    exec(27);
    exec(25);
    exec(19);
}
