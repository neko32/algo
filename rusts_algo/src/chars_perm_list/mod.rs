
pub fn exec(s:&str) -> Vec<String> {
    let mut v:Vec<String> = Vec::new();
    perm(Box::new(s.to_string()), Box::new(String::new()), &mut v);
    println!("input - {}", s);
    println!("perms are .. ");
    for x in v.iter() {
        println!("{}", x);
    }
    v
}

fn perm(s:Box<String>, t:Box<String>, v:&mut Vec<String>) {
    let sb = s.as_bytes();
    let len = s.len();
    println!("s-{:?}, t-{:?}", s, t);
    if len == 0 {
        v.push(t.to_string());
    }
    for i in 0..s.len() {
        let ch = sb[i];
        let a = String::from_utf8_lossy(&sb[0..i]).to_string();
        let b = String::from_utf8_lossy(&sb[i + 1..len]).to_string();
        let remain = format!("{}{}", a, b);
        let newt = format!("{}{}", t, ch as char);
        perm(Box::new(remain), Box::new(newt), v);
    }
}

pub fn run() {
    exec("ABCD");
}