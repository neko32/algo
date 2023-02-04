
pub fn exec(s:&str) -> Result<String, anyhow::Error> {
    let bs = s.as_bytes();
    let mut buf:Vec<u8> = Vec::new();
    for c in bs {
        if *c as char == ')' {
            let mut t:Vec<u8> = Vec::new();
            loop {
                if buf.len() == 0 {
                    return Err(anyhow::Error::msg("no corresponding paren"));
                }
                let popped = buf.pop().unwrap();
                if popped as char == '(' {
                    break;
                }
                t.push(popped);
            }
            for t_v in t {
                buf.push(t_v);
            }
        } else {
            buf.push(*c);
        }
    }

    let rez = String::from_utf8(buf).unwrap();
    println!("input - {s}, result - {rez}");
    Ok(rez)
}

pub fn run() {
    println!("{}", exec("(bar)").unwrap());
    println!("{}", exec("foo(bar)baz").unwrap());
    println!("{}", exec("foo(bar)baz(blim)").unwrap());
    println!("{}", exec("foo(bar(baz))blim").unwrap());
}
