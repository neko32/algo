
use std::collections::HashSet;
pub fn exec(v:Vec<Vec<i32>>) -> u32 {
    let r = v.len();
    let c = v[0].len();
    let mut s:HashSet<Vec<i32>> = HashSet::new();
    for i in 1..r {
        for j in 1..c {
            s.insert(run_probe(&v, i, j));
        }
    }
    println!("result - {:?}, size - {}", &s, s.len());
    s.len() as u32
}

fn run_probe(v:&Vec<Vec<i32>>, r:usize, c:usize) -> Vec<i32> {
    let mut buf:Vec<i32> = Vec::new();
    for i in r - 1..=r {
        for j in c - 1..=c {
            buf.push(v[i][j]);
        }
    }
    println!("{:?}", &buf);
    buf
}

pub fn run() {
    let v:Vec<Vec<i32>> = vec![
        vec![1, 2, 1],
        vec![2, 2, 2],
        vec![2, 2, 2],
        vec![1, 2, 3],
        vec![2 ,2, 1],
    ];
    exec(v);
}