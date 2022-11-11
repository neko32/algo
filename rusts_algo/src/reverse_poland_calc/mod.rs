pub mod reverse_poland_calc {

    pub fn exec(s: &str) -> f32 {
        let sb = s.as_bytes();
        let mut rp: Vec<f32> = Vec::new();
        println!("s = {}", s);
        for c in sb {
            let ch = *c as char;
            if ch.is_ascii_digit() {
                let c = ch.to_digit(10).unwrap() as f32;
                rp.push(c);
            } else {
                let o2 = rp.pop().unwrap();
                let o1 = rp.pop().unwrap();
                let mut total: f32 = 0_f32;
                if ch == '+' {
                    total = o1 + o2;
                    println!("{}+{}={}", o1, o2, total);
                } else if ch == '-' {
                    total = o1 - o2;
                    println!("{}-{}={}", o1, o2, total);
                } else if ch == '*' {
                    total = o1 * o2;
                    println!("{}*{}={}", o1, o2, total);
                } else if ch == '/' {
                    total = o1 / o2;
                    println!("{}/{}={}", o1, o2, total);
                } else {
                    panic!("not supported operator");
                }
                rp.push(total);
            }
        }
        rp.pop().unwrap()
    }

    pub fn run() -> () {
        let s = "12+3+4+";
        println!("{} => {}", s, exec(s));
    }
}
