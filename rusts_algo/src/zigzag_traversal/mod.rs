
use ndarray::{Array2, arr2};

pub fn exec(m:&Array2<i32>) -> Vec<i32> {
    let mut v:Vec<i32> = Vec::new();
    let mut direction_south = true;
    let mut x = 0;
    let mut y = 0;
    let x_max = m.shape()[1] - 1;
    let y_max = m.shape()[0] - 1;
    println!("input siz - {:?}", m.shape());
    println!("input - {}", m.view());

    while !(x == x_max && y == y_max) {
        println!("@({x},{y}) - {}", m[(y, x)]);
        v.push(m[(y, x)]);
        if direction_south {
            if x == 0 || y == y_max {
                if y == y_max {
                    x += 1;
                    direction_south = false;
                } else {
                    y += 1;
                    direction_south = false;
                }
            } else {
                y += 1;
                x -= 1;
            }
        } else {
            if y == 0 || x == x_max {
                if x == x_max {
                    y += 1;
                    direction_south = true;
                } else {
                    x += 1;
                    direction_south = true;
                }
            } else {
                x += 1;
                y -= 1;
            }
        }
    }
    v.push(m[(y_max, x_max)]);
    println!("result - {:?}", v);
    v
}


pub fn run() {
    let m:Array2<i32> = arr2(&[
        [1, 3, 4, 10],
        [2, 5, 9, 11],
        [6, 8, 12, 15],
        [7, 13, 14, 16],
    ]);
    exec(&m);
}
