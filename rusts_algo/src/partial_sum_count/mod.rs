
use ndarray::Array2;

pub fn exec(v:&[i32], t:i32) -> i32 {
    let len = v.len();
    let mut dp:Array2<i32> = Array2::zeros([len + 1, t as usize + 1]);
    dp[[0, 0]] = 1;
    for i in 0..len {
        for tval in 0..=t {
            if v[i] <= tval {
                dp[[i + 1, tval as usize]] = dp[[i, (tval - v[i]) as usize]] + dp[[i, tval as usize]];
            } else {
                dp[[i + 1, tval as usize]] = dp[[i, tval as usize]];
            }
        }
    }

    let rez = dp[[len, t as usize]];
    println!("input - {:?}, t - {t}, result - {rez}", v);
    rez
}

pub fn run() {
    exec(&[1, 2, 1, 3, 2], 4);
}
