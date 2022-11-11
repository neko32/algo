pub mod string_pattern {

    pub fn exec(s: &str) -> String {
        let mut buf = String::new();
        let sb = s.as_bytes();
        let mut idx = 0;
        let len = s.len();
        while idx < len {
            let mut k = idx + 1;
            let mut streak = 1;
            while k < len && sb[idx] == sb[k] {
                k += 1;
                streak += 1;
            }
            let t = format!("{}{}", streak, sb[idx] as char);
            buf.push_str(t.as_str());
            idx = k;
        }
        buf
    }

    pub fn run() -> () {
        let input = "33372211115";
        println!("{}", exec(input));
    }
}
