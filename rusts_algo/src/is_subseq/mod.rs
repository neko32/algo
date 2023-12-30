
pub fn exec(l:&[i32], s:&[i32]) -> bool {
    let mut lidx:usize = 0;
    let mut sidx:usize = 0;
    while lidx < l.len() && sidx < s.len() {
        if l[lidx] == s[sidx] {
            lidx += 1;
            sidx += 1;
        } else {
            lidx += 1;
        }
    }
    let r = sidx == s.len();
    println!("{:?} is subseq of {:?}? - {}", s, l, r);
    r
}

pub fn run() {
    exec(&[5, 1, 22, 25, 6, -1, 8, 10], &[1, 6, -1, 10]);
    exec(&[5, 1, 22, 25, 6, -1, 8, 10], &[1, -1, 6, 10]);
    exec(&[3, 5], &[2, 8, 7]);
    exec(&[3, 5], &[3, 5, 7]);
    exec(&[3, 5, 7], &[3, 5, 7]);
    exec(&[3, 5, 7], &[3, 5]);
    exec(&[3, 5, 7], &[3, 7]);
    exec(&[3, 5, 7], &[5, 7]);
    exec(&[3, 5, 7], &[7, 5]);
    exec(&[3, 5, 7], &[7, 3]);
    exec(&[3, 5, 7], &[7, 5, 3]);
}
