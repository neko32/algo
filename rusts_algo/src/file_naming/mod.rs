
use std::collections::HashMap;
pub fn exec(names:Vec<&str>) -> Vec<String> {
    let mut rez:Vec<String> = Vec::new();
    let mut h:HashMap<String, i32> = HashMap::new();
    for name in &names {
        match h.get(*name) {
            Some(cnt) => {
                let mut maybe_new_name = format!("{}({})", name, cnt);
                let mut cnt = *cnt;
                while rez.contains(&maybe_new_name) {
                    cnt += 1;
                    maybe_new_name = format!("{}({})", name, cnt);
                }
                rez.push(maybe_new_name.clone());
                h.insert(maybe_new_name, cnt);
            },
            None => {
                rez.push(name.to_string());
                h.insert(name.to_string(), 1);
            }
        }
    }
    println!("input - {:?}", names);
    println!("result - {:?}", rez);
    rez
}

pub fn run() {
    let names = vec!["doc", "doc", "image", "doc(1)", "doc"];
    exec(names);
}