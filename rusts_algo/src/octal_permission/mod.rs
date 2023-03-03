
use std::collections::HashMap;

pub fn exec(perm: &str) -> HashMap<String, String> {
    let mut h:HashMap<String, String> = HashMap::new();
    for (idx, b) in perm.as_bytes().iter().enumerate() {
        let c = *b as char;
        h.insert(derive_key(idx), parse(*b));
    }
    println!("input - {perm}, result - {:?}", h);
    h
}

fn derive_key(idx:usize) -> String {
    if idx == 0 {
        "Owner".to_string()
    } else if idx == 1 {
        "Group".to_string()
    } else {
        "Other".to_string()
    }
}

fn parse(v:u8) -> String {
    let mut buf = String::new();
    if v & 0x4 == 4 {
        buf.push_str("r");
    }
    if v & 0x2 == 2 {
        buf.push_str("w");
    }
    if v & 0x01 == 1 {
        buf.push_str("x");
    }
    buf
}

pub fn run() {
    exec("742");
    exec("777");
    exec("600");
    exec("631");
    exec("554");
}
