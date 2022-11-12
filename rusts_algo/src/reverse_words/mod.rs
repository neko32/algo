
use std::collections::VecDeque;

pub fn exec(s:&str) -> String {
    let mut q:VecDeque<String> = VecDeque::new();

    let mut idx = 0;
    let sb = s.as_bytes();
    let len = sb.len();
    while idx < len {
        let mut k = idx + 1;
        while k < len && sb[k] as char != ' ' {
            k += 1;
        }
        let w = String::from_utf8_lossy(&sb[idx..k]).to_string();
        q.push_front(w);
        idx = k + 1;
    }

    let v:Vec<String> = q.into();
    let rez = v.join(" ");
    println!("input - {}, output - {}", &s, &rez);
    rez
}

pub fn run() {
    let s = "neko ha kawaii sugoku kawaii!";
    exec(s);
}