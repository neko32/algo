
use ndarray::{arr2, Array2};
pub fn exec(m:&Array2<bool>) -> Array2<i32> {
    let ms = m.shape();
    let r = ms[0];
    let c = ms[1];
    let mut rez:Array2<i32> = Array2::zeros([r, c]);
    for i in 0..r {
        for j in 0..c {
            rez[[i, j]] = calc(&m, i as i32, j as i32, r as i32, c as i32);
        }
    }
    println!("input - \n{}", m.view());
    println!("result - \n {}",rez.view());
    rez
}

fn calc(m:&Array2<bool>, i:i32, j:i32, r:i32, c:i32) -> i32 {
    let mut cnt = 0;
    let cur_loc_x = i as usize;
    let cur_loc_y = j as usize;
    let i_st = (if i - 1 < 0 { 0 } else { i - 1 }) as usize;
    let i_ed = (if i + 1 == r { r - 1 } else { i + 1 }) as usize;
    let j_st = (if j - 1 < 0 { 0 } else { j - 1 }) as usize;
    let j_ed = (if j + 1 == c { c - 1 } else { j + 1 }) as usize;
    println!("@{cur_loc_x},{cur_loc_y}");
    println!("i:{i_st} -> {i_ed}, j:{j_st} -> {j_ed}");
    for x in i_st..=i_ed {
        for y in j_st..=j_ed {
            if x == cur_loc_x && y == cur_loc_y {
                // skip
            } else if m[[x, y]] {
                cnt += 1;
            }
        }
    }
    cnt
}

pub fn run() {
    let m:Array2<bool> = arr2(&[
        [true, false, false],
        [false, true, false],
        [false, false, false]
    ]);
    exec(&m);
}
