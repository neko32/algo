
use ndarray::{arr2, Array2};
use std::collections::VecDeque;
pub fn exec(m:&mut Array2<i32>) -> i32 {
    let mut q:VecDeque<(usize, usize)> = VecDeque::new();
    let mut t:VecDeque<(usize, usize)> = VecDeque::new();

    scan_all_non_negative(m, &mut q);
    let mut cnt = 1;

    while q.len() > 0 {
        while let Some((x, y)) = q.pop_front() {
            for (n_x, n_y) in get_neighbor(m, x, y) {
                m[(n_x, n_y)] = m[(n_x, n_y)].abs();
                t.push_back((n_x, n_y));
            }
        }
        while let Some((x, y)) = t.pop_front() {
            q.push_back((x, y));
        }
        println!("@after {}", cnt);
        println!("{}", m.view());
        cnt += 1;
    }

    let negative_cnt = count_all_negative(m);
    println!("final result - negative count is {negative_cnt}");
    println!("{}", m.view());
    println!("num of pass(es) - {}", cnt - 1);

    if negative_cnt == 0 {
        cnt - 1
    } else {
        -1
    }
}

fn get_neighbor(m:&Array2<i32>, loc_x:usize, loc_y:usize) -> Vec<(usize, usize)> {
    let mut v:Vec<(usize, usize)> = Vec::new();
    let (x_len, y_len) = m.dim();
    if loc_x > 0 && m[(loc_x - 1, loc_y)] < 0 {
        v.push((loc_x - 1, loc_y));
    }
    if loc_y > 0 && m[(loc_x, loc_y - 1)] < 0 {
        v.push((loc_x, loc_y - 1));
    }
    if loc_x < x_len - 1 && m[(loc_x + 1, loc_y)] < 0 {
        v.push((loc_x + 1, loc_y));
    }
    if loc_y < y_len - 1 && m[(loc_x, loc_y + 1)] < 0 {
        v.push((loc_x, loc_y + 1));
    }
    v
}

fn count_all_negative(m:&Array2<i32>) -> u32 {
    let mut count = 0_u32;
    let (c_siz, r_siz) = m.dim();
    for i in 0..c_siz {
        for j in 0..r_siz {
            if m[(i, j)] < 0 {
                count += 1;
            }
        }
    }
    count
}

fn scan_all_non_negative(m:&Array2<i32>, q:&mut VecDeque<(usize, usize)>) {
    let (c_siz, r_siz) = m.dim();
    for i in 0..c_siz {
        for j in 0..r_siz {
            if m[(i, j)] >= 0 {
                q.push_back((i, j));
            }
        }
    }
}

pub fn run() {
    let mut m:Array2<i32> = arr2(&[
        [0, -1, -3, 2, 0],
        [1, -2, -5, -1, -3],
        [3, 0, 0, -4, -1]
    ]);
    exec(&mut m);
}
