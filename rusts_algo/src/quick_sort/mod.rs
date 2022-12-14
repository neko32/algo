
pub fn exec(v:&mut Vec<i32>, l:usize, r:usize) {
    if l >= r {
        return;
    }
    let mut left = l;
    let mut right = r;
    let m = (l + r) / 2;
    let midval = v[m];

    loop {
        while v[left] < midval {
            left += 1;
        }
        while v[right] > midval {
            right -= 1;
        }
        if left >= right {
            break;
        }
        v.swap(left, right);
        println!("left-{},right-{}", left, right);
        left += 1;
        right -= 1;
    }

    exec(v, l, left - 1);
    exec(v, right + 1, r);
}

pub fn run() {
    let mut v = vec![7, 3, 10, 5, 25, 1, 9, 10, 6, 17, 12];
    let len = v.len();
    println!("before - {:?}", v);
    exec(&mut v, 0, len - 1);
    println!("after - {:?}", v);
}