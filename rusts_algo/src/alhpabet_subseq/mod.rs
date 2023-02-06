

pub fn exec(s:&str) -> bool {
    let bs = s.as_bytes();
    for i in 1..s.len() {
        if bs[i - 1] >= bs[i] {
            return false;
        }
    }
    true
}

pub fn run() {
    chk("effg");
    chk("cdce");
    chk("ace");
    chk("bxz");
    chk("abcdexyz");
    chk("abcdexmyz");
}

fn chk(s:&str) {
    let rez = exec(s);
    println!("{s} - {rez}");
}
