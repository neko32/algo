
use hmap::hmap;
use std::collections::HashMap;
pub fn exec(phone_num:&str, table:&HashMap<char, String>) -> Vec<String> {
    let mut buf:Vec<String> = Vec::new();
    trav(phone_num, 0, "".to_string(), &mut buf, table);
    println!("input - {}, result - {:?}", phone_num, buf);
    buf
}

fn trav(n:&str,
        idx:usize,
        s:String,
        buf:&mut Vec<String>,
        table:&HashMap<char, String>) {
    if idx == n.len() {
        println!("{}", s);
        buf.push(s);
        return;
    }

    let bn = n.as_bytes();
    let key = bn[idx] as char;
    let empty_s = format!("{}", key);
    let mnemonic = table.get(&key).unwrap_or(&empty_s);
    println!("{}-{}", key, mnemonic);
    for c in mnemonic.chars() {
        trav(n, idx + 1, format!("{}{}", s, c), buf, table);
    }
}

pub fn run() {
    let table:HashMap<char, String> = hmap!(
        '2' => "abc".to_string(),
        '3' => "def".to_string(),
        '4' => "ghi".to_string(),
        '5' => "jkl".to_string(),
        '6' => "mno".to_string(),
        '7' => "pqrs".to_string(),
        '8' => "tuv".to_string(),
        '9' => "wxyz".to_string()
    );
    exec("1905", &table);
}

