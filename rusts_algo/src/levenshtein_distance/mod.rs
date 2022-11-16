
use multiarray::Array2D;
use std::cmp::min;

pub fn exec(a:&str, b:&str) -> u32 {
    let ba = a.as_bytes();
    let bb = b.as_bytes();
    let alen = ba.len();
    let blen = bb.len();

    let mut dp = Array2D::new([alen + 1, blen + 1], 0_u32);
    for i in 0..=alen {
        for j in 0..=blen {
            dp[[i, j]] = j as u32;
        }
    }
    for i in 1..alen {
        dp[[i, 0]] = dp[[i - 1, 0]] + 1;
    }

    for i in 0..alen {
        for j in 0..blen {
            if ba[i] as char == bb[j] as char {
                dp[[i + 1, j + 1]] = dp[[i, j]];
            } else {
                dp[[i + 1, j + 1]] = 1 + min(dp[[i, j]], dp[[i + 1, j]].min(dp[[i, j + 1]]));
            }
        }
    }

    for i in 0..=alen {
        for j in 0..=blen {
            print!("{} ", dp[[i, j]]);
        }
        println!("");
    }

    dp[[alen, blen]]

}

pub fn run() {
    let s = "abc";
    let t = "yabd";
    println!("{} diff {} - {}", s, t, exec(s, t));
}
