
pub fn exec(v:&[i32]) -> (f32, f32) {
    let len = v.len();
    let m = len / 2;
    println!("input - {:?}", v);
    if len == 1 {
        println!("lower hinge - {}, upper hinge - {}", v[0], v[0]);
        return (v[0] as f32, v[0] as f32);
    } else if len == 2 {
        println!("lower hinge - {}, upper hinge - {}", v[0], v[1]);
        return (v[0] as f32, v[1] as f32);
    } else if len == 3 {
        println!("lower hinge - {}, upper hinge - {}", v[0], v[2]);
        return (v[0] as f32, v[2] as f32);
    }
    if len % 2 == 1 {
        if (m + 1) / 2 == 0 {
            let l = (v[m / 2] + v[(m + 1) / 2]) as f32 / 2.;
            let r = (v[m + m / 2] + v[m + (m + 1) / 2]) as f32 / 2.;
            println!("lower hinge - {l}, upper hinge - {r}");
            (l, r)
        } else {
            let l = v[m / 2] as f32;
            let r = v[m + (m / 2)] as f32;
            println!("lower hinge - {l}, upper hinge - {r}");
            (l, r)
        }
    } else {
        let (a, b) = v.split_at(m);
        let mm = a.len() / 2;
        // println!("{:?},{:?} - {}", a, b, mm);
        let l = (a[mm - 1] as f32 + a[mm] as f32) / 2.;
        let r = (b[mm - 1] as f32 + b[mm] as f32) / 2.;
        println!("lower hinge - {l}, upper hinge - {r}");
        (l, r)
    }
}

pub fn run() {
    exec(&[12, 13, 14, 15, 17]);
    exec(&[1, 2, 3]);
    exec(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    exec(&[1, 2]);
    exec(&[1, 2, 3, 4]);
    exec(&[1, 2, 3, 4, 5, 6, 7, 8]);
}
