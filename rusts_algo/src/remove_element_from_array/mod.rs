
pub fn exec(a:&mut Vec<i32>, b:i32) -> usize {
    let mut s = 0;
    let mut k = 0;
    let mut u = 0;
    let len = a.len();
    println!("before:{:?}, to remove - {}", a, b);
    while s < len {
        k = s;
        while k < len && a[k] == b {
            k += 1;
        }
        if k < len {
            a[u] = a[k];
            u += 1;
        }
        s = k + 1;
    }

    println!("after:{:?}, letsiz - {}", a, u);
    u
} 

pub fn run() {
    exec(&mut vec![3, 2, 2, 3], 3);
    exec(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2);
}

pub fn verify(a:&Vec<i32>, siz:usize, b:i32) -> bool {
    a[0..siz].iter().all(|&x|x != b)
}