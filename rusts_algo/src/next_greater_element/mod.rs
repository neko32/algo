
pub fn exec(a:&[i32]) -> Vec<i32> {
    let mut idx = 0;
    let len = a.len();
    let mut rez:Vec<i32> = Vec::from_iter(std::iter::repeat(0).take(len));

    while idx < len {
        let mut k = (idx + 1) % len;
        println!("starting at {} for idx {}", k, idx);
        while k != idx && a[idx] >= a[k] {
            k = (k + 1) % len;
        }
        if k == idx {
            rez[idx] = -1;
        } else {
            rez[idx] = a[k];
        }
        idx += 1;
    }
    println!("input - {:?}, result - {:?}", a, rez);
    rez
}

pub fn run() {
    let a = &[2, 5, -3, -4, 6, 7, 2];
    exec(a);
}