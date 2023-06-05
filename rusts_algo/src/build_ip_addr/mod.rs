
pub fn exec(s:&str) -> Result<Vec<String>, anyhow::Error> {
    let mut v:Vec<String> = Vec::new();
    let bs = s.as_bytes();
    let len = bs.len();

    for i in 1..=3 {
        let a = &bs[0..i];
        let a_s = String::from_utf8_lossy(a).to_string();
        println!("a:{a_s}");
        let mut buf:Vec<String> = Vec::new();
        if let Ok(a_r) = is_valid_ip(a_s.as_str()) {
            if a_r {
                let j_s = i + 1;
                let j_e = j_s + 3.min(len - j_s);
                for j in j_s..j_e {
                    let b = &bs[i..j];
                    let b_s = String::from_utf8_lossy(b).to_string();
                    println!("b:{b_s}");
                    if let Ok(b_r) = is_valid_ip(b_s.as_str()) {
                        if b_r {
                            let k_s = j + 1;
                            let k_e = k_s + 3.min(len - k_s);
                            println!("XXX:{k_s}:{k_e}");
                            for k in k_s..k_e {
                                let c = &bs[j..k];
                                let c_s = String::from_utf8_lossy(c).to_string();
                                println!("c:{c_s}");
                                let d = &bs[k..];
                                let d_s = String::from_utf8_lossy(d).to_string();
                                println!("d:{d_s}");
                                if let Ok(c_r) = is_valid_ip(c_s.as_str()) {
                                    if c_r {
                                        if let Ok(d_r) = is_valid_ip(d_s.as_str()) {
                                            if d_r {
                                                buf.push(a_s.clone());
                                                buf.push(b_s.clone());
                                                buf.push(c_s);
                                                buf.push(d_s);
                                                let valid_ip = buf.join(".");
                                                println!("GOT VALID IP! - {valid_ip}");
                                                v.push(valid_ip);
                                                buf.clear();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("input - {s}, result - {:?}", v);
    Ok(v)
}

fn is_valid_ip(s:&str) -> Result<bool, anyhow::Error> {
    let n:i32 = s.parse()?;
    if n < 0 || n > 255 {
        Ok(false)
    } else {
        let ss = n.to_string();
        Ok(s == ss)
    }
}

pub fn run() {
    let s = "1921680";
    exec(s).unwrap();
}
