
pub fn exec(v:&mut Vec<i32>) {
    let mut l = 0;
    let mut r = v.len() - 1;
    println!("before - {:?}", &v);
    while v[l] == -1 {
        l += 1;
    }
    while v[r] == -1 {
        r -= 1;
    }

    while l < r {
        println!("at {}", l);
        let mut min = v[l];
        let mut minidx = l;
        for i in l + 1..=r {
            if v[i] != -1 && v[i] < min {
                minidx = i;
                min = v[i];
            }
        }
        println!("base {}@{}, to swap with {}@{}", v[l], l, v[minidx], minidx);
        v.swap(l, minidx);
        l += 1;
        while v[l] == -1 && l < v.len() {
            l += 1;
        }
    }
    println!("after - {:?}", &v);
}

pub fn run() {
    let mut v = vec![-1, 150, 190, 170, -1, -1, 160, 180];
    exec(&mut v);
}