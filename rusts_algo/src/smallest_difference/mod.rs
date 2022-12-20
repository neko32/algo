
pub fn exec(oa:&[i32], ob:&[i32]) -> (i32, i32) {

    let mut a:Vec<i32> = Vec::new();
    let mut b:Vec<i32> = Vec::new();
    a.extend_from_slice(oa);
    b.extend_from_slice(ob);
    a.sort();
    b.sort();

    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut smallest_pair = (i32::MAX, i32::MAX);
    let mut smallest_v = i32::MAX;
    let mut diff = 0;
    while a_idx < a.len() && b_idx < b.len() {
        let av = a[a_idx];
        let bv = b[b_idx];
        if av < bv {
            diff = bv - av;
            a_idx += 1;
        } else {
            diff = av - bv;
            b_idx += 1;
        }

        if diff == 0 {
            return (av, bv);
        }

        if smallest_v > diff {
            smallest_v = diff;
            smallest_pair = (av, bv);
        }
    }
    smallest_pair
}

pub fn run() {
    let a = [-1, 5, 10, 20, 28, 3];
    let b = [26, 134, 135, 15, 17];
    let rez = exec(&a, &b);
    println!("input a - {:?}, b - {:?}, result - {:?}", a, b, rez);
}