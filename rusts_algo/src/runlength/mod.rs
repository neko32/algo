
pub mod runlength {

    pub fn exec(s:String) -> String {
        let mut idx:usize = 0;    
        let len = s.len();
        let sb = s.as_bytes();
        let mut buf:String = String::new();
        while idx < len {
            let mut k = idx + 1;
            let mut streak = 1;
            while k < len && sb[idx] as char == sb[k] as char && streak < 9 {
                k += 1;    
                streak += 1;
            }

            buf.push_str(format!("{}{}", streak, sb[idx] as char).as_str());
            idx = k;
        }
        buf
    }

    pub fn run() {
        let s = "AAAAAAAAAAAA";    
        let rez = exec(s.to_string());
        println!("s={}, rez={}", s, rez);
    }
}