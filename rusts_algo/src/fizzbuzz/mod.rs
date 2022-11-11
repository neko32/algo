pub mod fizzbuzz {

    pub fn exec(n: u16) -> String {
        let mut rez: String = String::new();
        for i in 1..=n {
            let mut buf: String = String::new();
            if i % 3 == 0 {
                buf = "fizz".to_string();
            }
            if i % 5 == 0 {
                buf.push_str("buzz");
            }
            if buf.len() == 0 {
                buf = i.to_string();
            }
            rez.push_str(buf.as_str());
        }
        println!("fizzbuzz({}) = {}", n, &rez);
        rez
    }

    pub fn run() {
        let n = 30_u16;
        exec(n);
    }
}
