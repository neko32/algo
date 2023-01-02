
use ndarray::{arr2, Array2};

pub fn exec(matrix:&mut Array2<i32>) {
    let shape = matrix.shape();
    let x_siz = shape[0];
    let y_siz = shape[1];
    for x in 0..x_siz {
        for y in 0..y_siz {
            if is_at_border(x, y, &matrix) && matrix[(x, y)] == 1 {
                println!("at border, start traversing..");
                trav_and_update(matrix, x, y);
            }
        }
    }

    for x in 0..x_siz {
        for y in 0..y_siz {
            let v = matrix[(x, y)];
            match v {
                2 => matrix[(x, y)] = 1,
                1 => matrix[(x, y)] = 0,
                _ => (),
            }
        }
    }
}

fn trav_and_update(matrix:&mut Array2<i32>, x:usize, y:usize) {
    println!("trav_and_update@({x},{y})");
    matrix[(x, y)] = 2;
    let chk = |(a,b)|{matrix[(a, b)] == 1};
    for neighbor in get_neighbors(&matrix, x, y) {
        println!("neighbor to go is {:?}", neighbor);
        trav_and_update(matrix, neighbor.0, neighbor.1);
    }
}

fn get_neighbors(matrix:&Array2<i32>, x:usize, y:usize) -> Vec<(usize, usize)> {
    let shape = matrix.shape();
    let x_max = (shape[0] - 1) as i32;
    let y_max = (shape[1] - 1) as i32;
    let ix = x as i32;
    let iy = y as i32;
    let chk = |(a,b)|{matrix[(a, b)] == 1};
    let mut neighbors:Vec<(usize, usize)> = Vec::new();
    let ranges = vec![(ix - 1, iy), (ix + 1, iy), (ix, iy - 1), (ix, iy + 1)];
    for (i, j) in ranges {
        if i > 0 && j > 0 && i < x_max && j < y_max && chk((i as usize, j as usize)) {
            neighbors.push((i as usize, j as usize));
        }
    }
    neighbors
}

fn is_at_border(x_idx:usize, y_idx:usize, m:&Array2<i32>) -> bool {
    let s = m.shape();
    let x = s[0];
    let y = s[1];
    let x_max = x - 1;
    let y_max = y - 1;
    x_idx == 0 || y_idx == 0 || x_idx == x_max || y_idx == y_max
}

pub fn run() {
    let mut matrix = arr2(&[
        [1, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 1, 1],
        [0, 0, 1, 0, 1, 0],
        [1, 1, 0, 0, 1, 0],
        [1, 0, 1, 1, 0, 0],
        [1, 0, 0, 0, 0, 1],
    ]);
    println!("before - \n{}", matrix.view());
    exec(&mut matrix);
    println!("after - \n{}", matrix.view());
}
