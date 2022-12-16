
use ndarray::{arr2, Array2};
pub fn exec(matrix:&Array2<u32>, target:u32) -> (i32, i32) {
    let shape = matrix.shape();
    let rlen = shape[0];
    let clen = shape[1];
    let mut c = (clen - 1) as i32;
    let mut r = 0_usize as i32;

    let mut rez:Option<(i32, i32)> = None;
    while !is_in_bound(r, c, rlen, clen) {
        if matrix[(r as usize, c as usize)] == target {
            rez = Some((r, c));
            break;
        } else if matrix[(r as usize, c as usize)] > target {
            c -= 1;
        } else {
            r += 1;
        }
    }
    let rez = rez.unwrap_or((-1, -1));
    println!("input matrix - {}", matrix.view());
    println!("result - {:?}", rez);
    rez
}

fn is_in_bound(r:i32, c:i32, rlen:usize, clen:usize) -> bool {
    r < 0 || c < 0 || r as usize >= rlen || c as usize >= clen
}

pub fn run() {
    let matrix:Array2<u32> = arr2(&[
        [1, 4, 7, 12, 15, 1000],
        [2, 5, 19, 31, 32, 1001],
        [3, 8, 24, 33, 35, 1002],
        [40, 41, 42, 44, 45, 1003],
        [99, 100, 103, 106, 128, 1004],
    ]);
    exec(&matrix, 44);
}