

pub fn exec(a:&[i32]) -> Option<bool> {
    let mut state = State::TBD;
    let mut idx = 1;
    let len = a.len();
    while state == State::TBD && idx < len {
        if a[idx] > a[idx - 1] {
            state = State::INCREASING;
            break;
        } else if a[idx] < a[idx - 1] {
            state = State::DECREASING;
            break;
        }
        idx += 1;
    }
    if state == State::TBD {
        None
    } else {
        for i in idx + 1..len {
            match state {
                State::DECREASING => {
                    if a[i] >= a[i - 1] {
                        return Some(false);
                    }
                },
                State::INCREASING => {
                    if a[i] <= a[i - 1] {
                        return Some(false);
                    }
                },
                _ => panic!("shouldn't happen"),
            }
        }
        Some(true)
    }
}

#[derive(Eq, PartialEq, Debug)]
enum State {
    TBD,
    INCREASING,
    DECREASING,
}

pub fn run() {
    let a = [3, 7, 10, 25, 50, 100, 150];
    let b = [100, 72, 55, 40, 32, 10, 3, 1];
    let c = [150, 125, 72, 138, 50, 62, 22];
    let d = [1, 1, 1, 1, 2, 3, 5, 10];
    let e = [10, 10, 10, 7, 4, 3, 1];
    let f = [5, 5, 5, 5, 5, 5, 5];
    println!("input - {:?}, rez - {:?}", a, exec(&a));
    println!("input - {:?}, {:?}", b, exec(&b));
    println!("input - {:?}, {:?}", c, exec(&c));
    println!("input - {:?}, {:?}", d, exec(&d));
    println!("input - {:?}, {:?}", e, exec(&e));
    println!("input - {:?}, {:?}", f, exec(&f));
}
