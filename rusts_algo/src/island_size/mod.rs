
use ndarray::{arr2, Array2};

pub fn exec(m_orig:&Array2<u8>) -> Vec<u32> {
    let mut rez:Vec<u32> = Vec::new();
    let mut m = m_orig.clone();
    let siz = m.shape();
    let r = siz[0];
    let c = siz[1];
    println!("input -\n {}", m.view());
    for i in 0..r {
        for j in 0..c {
            if m[[i, j]] == 1 {
                let ret_v = search(&mut m, i, j, 0);
                rez.push(ret_v);
            }
        }
    }
    println!("result - {:?}", rez);
    rez
}

fn search(m:&mut Array2<u8>, st_r:usize, st_c:usize, cnt:u32) -> u32 {
    let mut q:Vec<(usize, usize)> = Vec::new();
    let mut cnt = cnt;
    let (m_r_len, m_c_len) = m.dim();
    q.push((st_r, st_c));
    while !q.is_empty() {
        let (rr, cc) = q.pop().unwrap();
        cnt += 1;
        m[[rr, cc]] = 2;
        let r = rr as i32;
        let c = cc as i32;

        if r - 1 >= 0 && m[[rr - 1, cc]] == 1 {
            q.push((rr - 1, cc));
        }
        if r + 1 < m_r_len as i32 && m[[rr + 1, cc]] == 1 {
            q.push((rr + 1, cc));
        }
        if c - 1 >= 0 && m[[rr, cc - 1]] == 1 {
            q.push((rr, cc - 1));
        }
        if c + 1 < m_c_len as i32 && m[[rr, cc + 1]] == 1 {
            q.push((rr, cc + 1));
        }
    }
    cnt
}

pub fn run() {
    let mut m = arr2(&[
        [1, 0, 0, 1, 0],
        [1, 0, 1, 0, 0],
        [0, 0, 1, 0, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 1, 1, 0]
    ]);
    exec(&mut m);
}
