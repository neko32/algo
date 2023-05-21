
use ndarray::Array2;

pub fn exec(v:&[i32], k:i32, a:i32) -> bool {

    let len = v.len();
    let mut dp:Array2<i32> = Array2::from_elem([len + 1, (a + 1) as usize], i32::MAX - 1);
    dp[[0, 0]] = 0;
    for i in 0..len {
        for aval in 0..=a {
            if aval >= v[i] {
                let x = dp[[i, (aval - v[i]) as usize]] + 1;
                let y = dp[[i, aval as usize]];
                println!("{x} vs {y}");
                dp[[i + 1, aval as usize]] = x.min(y);
            } else {
                dp[[i + 1, aval as usize]] = dp[[i, aval as usize]];
            }
        }
    }
    println!("{}", dp.view());
    let rez = dp[[len, a as usize]] == k;
    println!("result - {rez}");
    rez
}

pub fn run() {
    exec(&[2, 3, 4, 5], 4, 14);
}

