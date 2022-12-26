
use ndarray::{Array2, arr2};

pub fn exec(matrix:&mut Array2<i32>, start_idx:i32, num_node:i32, num_edge:i32) -> Vec<Vec<i32>>{
    let mut t:Vec<i32> = Vec::new();
    let mut buf:Vec<Vec<i32>> = Vec::new();
    scan(start_idx, start_idx, num_node, matrix, num_edge, &mut t, &mut buf);
    buf
}

fn scan(idx:i32,
        start_idx:i32,
        num_node:i32,
        matrix:&mut Array2<i32>,
        num_edge:i32,
        t:&mut Vec<i32>,
        buf:&mut Vec<Vec<i32>>) {
    if idx == start_idx && num_edge == 0 {
        buf.push(t.clone());
    }

    for j in 0..num_node {
        match matrix[(idx as usize, j as usize)] {
            0 => (),
            _ => {
                t.push(idx);
                matrix[(idx as usize, j as usize)] -= 1;
                matrix[(j as usize, idx as usize)] -= 1;
                scan(j, start_idx, num_node, matrix, num_edge - 1, t, buf);
                matrix[(idx as usize, j as usize)] += 1;
                matrix[(j as usize, idx as usize)] += 1;
                t.pop();
            }
        }
    }
}

pub fn run() {
    let mut matrix:Array2<i32> = arr2(&[
        [0, 1, 0, 1],
        [1, 0, 1, 2],
        [0, 1, 0, 1],
        [1, 2, 1, 0],
    ]);
    let num_edges = 6;
    let num_nodes = 4;
    let r = exec(&mut matrix, 0, num_nodes, num_edges);
    println!("{:?}", r);
}
