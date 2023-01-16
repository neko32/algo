
pub fn exec(s:&mut Vec<i32>) {
    if s.len() > 0 {
        let popped = s.pop().unwrap();
        exec(s);
        do_sort(s, popped);
    }
}

pub fn do_sort(s:&mut Vec<i32>, popped_v:i32) {
    match s.last() {
        None => {
            s.push(popped_v);
        },
        Some(last_v) if *last_v < popped_v => {
            s.push(popped_v);
        },
        _ => {
            let t = s.pop().unwrap();
            do_sort(s, popped_v);
            s.push(t);
        }
    }
}

pub fn run() {
    let mut s:Vec<i32> = vec![7, 4, 8, 1, 5, 4, 10, 15, 3];
    println!("before - {:?}", s);
    exec(&mut s);
    println!("after - {:?}", s);
}
