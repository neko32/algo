
use std::collections::VecDeque;

pub fn exec(bldgs:&[u32], direction:&str) -> VecDeque<usize> {

    let mut r:VecDeque<usize> = VecDeque::new();
    let len = bldgs.len();
    let mut max_height = 0;

    match direction {
        d if d.to_lowercase() == "east" => {
            r.push_front(len - 1);
            max_height = bldgs[len - 1];
            for i in (0..=len - 2).rev() {
                if bldgs[i] > max_height {
                    r.push_front(i);
                    max_height = bldgs[i];
                }
            }
        },
        d if d.to_lowercase() == "west" => {
            r.push_back(0);
            max_height = bldgs[0];
            for i in 1..len {
                if bldgs[i] > max_height {
                    r.push_back(i);
                    max_height = bldgs[i];
                }
            }
        },
        _ => panic!("unknown direction"),
    }

    println!("buildings - {:?}, direction - {}, result - {:?}", bldgs, direction, r);
    r
}

pub fn run() {
    let bldgs1 = [3, 5, 4, 4, 3, 1, 3, 2];
    let bldgs2 = [2, 4, 4, 5, 1, 2, 8, 7, 10, 4];
    exec(&bldgs1, "east");
    exec(&bldgs2, "west");
}
