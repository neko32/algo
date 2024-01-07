use ndarray::{arr2, Array2};

pub fn exec(a:&Array2<i32>) -> Array2<i32> {
    let s = a.shape();
    let mut buf:Array2<i32> = Array2::zeros([s[0], s[1]]);

    for h in 0..s[0] {
        for w in 0..s[1] {
            let p = buf.get_mut((h, w)).unwrap();
            *p += scan(w + 1, h + 1, a);
        }
    }
    println!("{}", buf.view());
    buf
}

fn scan(w_siz:usize, h_siz:usize, a:&Array2<i32>) -> i32 {
    let mut sumv = 0;
    for h in 0..h_siz {
        for w in 0..w_siz {
            sumv += *a.get((h, w)).unwrap_or(&0);
        }
    }
    sumv
}

pub fn run() {
    let a:Array2<i32> = arr2(&[
        [1, 0, 0, 1, 0],
        [0, 0, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [1, 1, 0, 1, 0],
        [1, 0, 0, 0, 1]
    ]);
    exec(&a);
}