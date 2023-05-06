
use crossbeam::thread;

pub fn exec(l:&[i32]) -> i32 {
    let rez = bmax(l);
    println!("input - {:?}, result - {rez}", l);
    rez
}

fn bmax(l:&[i32]) -> i32 {
    let len = l.len();
    if len == 1 {
        l[0]
    } else if len == 2 {
        l[0].max(l[1])
    } else {
        let m = len / 2;
        let rez = thread::scope(|s| {
            let a = s.spawn(move |_| {
                bmax(&l[0..m])
            }).join().unwrap();
            let b = s.spawn(move |_| {
                bmax(&l[m..])
            }).join().unwrap();
            a.max(b)
        }).unwrap();
        rez
    }
}

pub fn run() {
    exec(&[72, 30, 1, 24, 9, 8, 132, -32, 50, 4]);
    exec(&[22]);
    exec(&[22, 72]);
    exec(&[30, 10, 20]);
    exec(&[40, 10, 50, 90, 10]);
}
