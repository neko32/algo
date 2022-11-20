

pub fn exec(a:&mut Vec<i32>, target:i32) -> Vec<Vec<i32>> {
    let mut idx = 0;
    a.sort();
    let len = a.len();
    let mut rez:Vec<Vec<i32>> = Vec::new();
    while idx < len - 2 {
        let mut l = idx + 1;
        let mut r = len - 1;
        while l < r {
            let sumv = a[idx] + a[l] + a[r];
            if sumv == target {
                rez.push(vec![a[idx], a[l], a[r]]);
                l += 1;
                r -= 1;
            } else if sumv < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        idx += 1;
    }
    println!("input - {:?}, target - {}", a, target);
    println!("result is {:?}", rez);
    rez
}

pub fn run() {
    let mut a = vec![12, 3, 1, 2, -6, 5, -8, 6];
    let target = 0;
    exec(&mut a, target);
}