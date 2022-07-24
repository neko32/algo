pub mod is_mac_addr {

    pub fn exec(s: &str) -> bool {
        if s.starts_with("-") || s.ends_with("-") {
            return false;
        } else {
            let parts: Vec<&str> = s.split_terminator("-").collect();
            if parts.len() != 6 {
                return false;
            }
            for part in parts {
                let partb = part.as_bytes();
                if partb.len() != 2 {
                    return false;
                }
                let rez = partb.iter().all(|c| {
                    let cc = *c;
                    (cc >= 65 && cc <= 70) || (cc >= 48 && cc <= 57)
                });
                if !rez {
                    return false;
                }
            }
            true
        }
    }

    pub fn run() -> () {
        let mac = "00-1B-63-84-45-E6";
        let bad = "00-G3-33-29-44-E6";
        println!("{}", exec(mac));
        println!("{}", exec(bad));
    }
}
