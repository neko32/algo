pub mod caesar_crypt {

    pub fn exec(s: &str, n: u8) -> String {
        let mut m = s.to_lowercase();

        println!("input str - {}, n - {}", s, n);
        unsafe {
            let mb = m.as_bytes_mut();
            for c in mb.iter_mut() {
                let t: i32 = (*c - 97) as i32;
                println!("{}", t);
                let x = (((t + n as i32) % 26) + 97) as u8;
                *c = x;
            }
            String::from_utf8_lossy(mb).to_string()
        }
    }

    pub fn run() -> () {
        println!("{}", exec("TAkonbiz", 3));
    }
}
