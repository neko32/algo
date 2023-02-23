
pub fn exec(s:&str) -> String {
    let mut ss = Vec::from_iter(s.chars());
    let mut idx = 0;
    let cs:Vec<char> = Vec::from_iter(s.chars());

    loop {
        let mut rev = ss.clone();
        rev.reverse();

        if ss == rev {
            break;
        }

        let ins_idx = ss.len() - 1 - idx;

        let mut sub1:Vec<char> = Vec::new();
        let mut sub2:Vec<char> = Vec::new();
        sub1.extend_from_slice(&ss[..=ins_idx]);
        sub2.extend_from_slice(&ss[ins_idx + 1..]);
        ss.clear();
        sub1.into_iter().for_each(|c|ss.push(c));
        ss.push(cs[idx]);
        sub2.into_iter().for_each(|c|ss.push(c));
        println!("ss - {:?}", ss);

        idx += 1;
    }

    let s:String = ss.into_iter().collect();
    println!("result - {s}");
    s

}

pub fn run() {
    exec("abcdc");
}