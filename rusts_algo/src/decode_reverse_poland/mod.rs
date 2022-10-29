
pub mod decode_reverse_poland {

    pub fn exec<'a>(s:&str) -> String {
        let mut buf:Vec<String> = Vec::new();
        let sb = s.as_bytes();
        for c in sb {
            let ch = *c as char;
            if ch.is_ascii_digit() {
                buf.push(ch.to_string());
            } else {
                let mut op2 = buf.pop().unwrap();
                let mut op1 = buf.pop().unwrap();
                if ch == '*' || ch == '/' {
                    if op1.len() > 1 {
                        op1 = format!("({})", op1);
                    }
                    if op2.len() > 1 {
                        op2 = format!("({})", op2);
                    }
                }
                if ch == '+' {
                    buf.push(format!("{}+{}", op1, op2));
                } else if ch == '-' {
                    buf.push(format!("{}-{}", op1, op2));
                } else if ch == '*' {
                    buf.push(format!("{}*{}", op1, op2));
                } else if ch == '/' {
                    buf.push(format!("{}/{}", op1, op2));
                } else {
                    // no op
                }
            }
        }

        buf.join("")
    }

    pub fn run() {
        let s = "612+*8-";
        let rez = exec(s);
        println!("{} -> {}", s, rez);
    }
}