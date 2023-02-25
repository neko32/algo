
use std::collections::HashMap;
pub fn exec(k:i32, tasks:Vec<i32>) -> Vec<(usize, usize)> {

    let mut rev_h = derive_effort_index_map(&tasks);
    let mut rez:Vec<(usize, usize)> = Vec::new();
    let len = tasks.len();

    let mut tasks_sorted = tasks.clone();
    tasks_sorted.sort();
    println!("tasks sorted -{:?}", tasks_sorted);
    println!("rev map - {:?}", rev_h);

    for i in 0..k {
        let first_t = tasks_sorted[i as usize];
        let first_indice = rev_h.get_mut(&first_t).unwrap();
        let first_idx = first_indice.pop().unwrap();

        let sec_t_idx = len - 1 - i as usize;
        let sec_t = tasks_sorted[sec_t_idx];
        let sec_indice = rev_h.get_mut(&sec_t).unwrap();
        let sec_idx = sec_indice.pop().unwrap();

        rez.push((first_idx, sec_idx));
    }

    println!("result - {:?}", rez);
    rez
}

fn derive_effort_index_map(tasks:&Vec<i32>) -> HashMap<i32, Vec<usize>> {
    let mut h:HashMap<i32, Vec<usize>> = HashMap::new();
    for (idx, effort) in tasks.iter().enumerate() {
        let p = h.entry(*effort).or_default();
        p.push(idx);
    }
    h
}

pub fn run() {
    exec(3, vec![1, 3, 5, 3, 1, 4]);
}