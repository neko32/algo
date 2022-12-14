
pub fn exec(j:&Vec<i32>, d:&Vec<Vec<i32>>) -> Option<Vec<i32>> {
    let mut visiting = Vec::from_iter(std::iter::repeat(false).take(100));
    let mut visited = Vec::from_iter(std::iter::repeat(false).take(100));
    let mut r:Vec<i32> = Vec::new();
    for start in j {
        println!("starting at {}", start);
        if sort(*start, &d, &mut r, &mut visited, &mut visiting) {
            return None
        }
        println!("start@{}'s esult {:?}", start, r);
    }
    r.reverse();
    Some(r)
}

fn sort(n:i32, d:&Vec<Vec<i32>>, r:&mut Vec<i32>, visited:&mut Vec<bool>, visiting:&mut Vec<bool>) -> bool {
    if visited[n as usize] {
        return false;
    }
    if visiting[n as usize] {
        return true;
    }

    visiting[n as usize] = true;

    let deps:Vec<i32> = d.iter().filter_map(|d|{
        if d[0] == n { Some(d[1])} else { None }
    }).collect();
    println!("{}'s dependencies are {:?}", n, deps);

    for nxt in deps {
        if sort(nxt, d, r, visited, visiting) {
            return true;
        }
    }

    visiting[n as usize] = false;
    visited[n as usize] = true;
    r.push(n);

    false
}


pub fn run() {
    let j = vec![1, 2, 3, 4];
    let mut deps:Vec<Vec<i32>> = Vec::new();
    deps.push(vec![1, 2]);
    deps.push(vec![1, 3]);
    deps.push(vec![3, 2]);
    deps.push(vec![4, 2]);
    deps.push(vec![4, 3]);
    let rez = exec(&j, &deps);
    match rez {
        Some(nodes) => println!("{:?}", nodes),
        None => println!("found closed loop"),
    }
}
