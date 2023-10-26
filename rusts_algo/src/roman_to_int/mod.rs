
use std::collections::HashMap;
use hashmap_macro::hashmap;

pub fn exec(s:&str) -> u32 {
    let mut rez = 0;
    let dict:HashMap<&str, u32> = hashmap!(
        "I" => 1, 
        "V" => 5,
        "X" => 10,
        "L" => 50,
        "C" => 100,
        "D" => 500,
        "M" => 1000,
        "IV" => 4,
        "IX" => 9,
        "XL" => 40,
        "XC" => 90,
        "CD" => 400,
        "CM" => 900,
    );

    let mut i:usize = 0;
    let len = s.len();
    while i < len {
        if i < len - 1 && dict.contains_key(&s[i..i + 2]) {
            rez += dict[&s[i..i + 2]];
            i += 2;
            continue;
        }
        rez += dict[&s[i..i + 1]];
        i += 1;
    }
    println!("{:?}:{}", s, rez);
    rez
}

pub fn run() {
    exec("III");
    exec("XIII");
    exec("VIV");
    exec("LVIII");
    exec("MCMXCIV");
}