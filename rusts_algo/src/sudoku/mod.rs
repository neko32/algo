
use ndarray::{arr2, Array2};

pub fn exec(m:&mut Array2<i32>) {
    println!("before - {}", m.view());
    let (r_siz, c_siz) = m.dim();
    println!("size: col {c_siz} and row:{r_siz}");
    if solve(m, 0, 0) {
        println!("solved - {}", m.view());
    } else {
        println!("couldn't solev - {}", m.view());
    }
}

fn solve(m:&mut Array2<i32>, c:usize, r:usize) -> bool {
    let (r_siz, c_siz) = m.dim();
    let mut r = r;
    let mut c = c;

    if c == c_siz {
        r += 1;
        c = 0;
        if r == r_siz {
            return true;
        }
    }

    if m[(r, c)] == -1 {
        return try_fill(m, c, r);
    }
    solve(m, c + 1, r)
}

fn try_fill(m:&mut Array2<i32>, c:usize, r:usize) -> bool {
    for i in 1..=9 {
        if is_valid(m, c, r, i) {
            println!("({c},{r}) = {i} is good.");
            m[(r, c)] = i;
            if solve(m, c + 1, r) {
                return true;
            }
        }
    }
    m[(r, c)] = -1;
    return false;
}

fn is_valid(m:&mut Array2<i32>, c:usize, r:usize, n:i32) -> bool {
    let has_n_in_raw =  m.row(r).iter().any(|x|*x == n);
    let has_n_in_col = m.column(c).iter().any(|x|*x == n);
    if has_n_in_raw || has_n_in_col {
        return false;
    }
    let r_base = (r / 3) * 3;
    let c_base = (c / 3) * 3;
    for i in r_base..r_base + 3 {
        for j in c_base..c_base + 3 {
            if m[(i, j)] == n {
                return false;
            }
        }
    }
    true
}

pub fn run() {
    let mut m = arr2(&[
        [-1, 1, 5, 9, -1, 4, -1, -1, -1],
        [6, 8, 3, -1, -1, 2, -1, -1, 7],
        [-1, 9, -1, -1, 3, -1, -1, -1, -1],
        [1, -1, -1, -1, 4, -1, -1, -1, 3],
        [9, 4, -1, -1, 6, 7, -1, 2, -1],
        [-1, 7, 2, -1, 8, 1, 4, -1, 6],
        [-1, 6, 1, -1, -1, 3, 2, 5, -1],
        [-1, 3, -1, 4, 2, 6, -1, 7, 1],
        [7, -1, -1, 8, 1, -1, -1, 6, 9]
    ]);
    exec(&mut m);
}
