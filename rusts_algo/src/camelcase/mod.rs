pub mod camelcase {

    pub fn exec(s: String) -> String {
        let mut s = s;
        unsafe {
            for (i, c) in s.as_bytes_mut().iter_mut().enumerate() {
                if i == 0 {
                    c.make_ascii_uppercase();
                } else {
                    c.make_ascii_lowercase();
                }
            }
        }
        println!("{}", s);
        s
    }

    pub fn run() {
        exec("elEpHAnT".to_string());
    }
}
