
pub fn exec(input:Vec<i32>) -> Vec<Vec<i32>> {

    let mut s:Vec<Vec<i32>> = Vec::new();
    let e:Vec<i32> = Vec::from([]);
    let mut t:Vec<Vec<i32>> = Vec::new();
    s.push(e);

    for v in input.iter() {
        for a in s.iter_mut() {
            let mut mutated = a.clone();
            mutated.push(*v);
            t.push(mutated);
        }
        while let Some(a) = t.pop() {
            s.push(a);
        }
    }

    println!("input - {:?}, result - {:?}", input, s);
    s
}

pub fn run() {
    exec(vec![1,2,3]);
}
