
pub mod cyclic_chars {

    pub fn exec(str:&str, n:u8) -> String {
        let mut buf:String = String::new();    
        let mut iter = str.chars().cycle();
        for _ in 0..n {
            buf.push(iter.next().unwrap());
        }
        buf
    }

    pub fn run() {
        let s = "ABC";
        let n = 7;
        let rez = exec(s, n);
        println!("n = {}, str = {}, then result is {}", n, s, rez);
    }
}