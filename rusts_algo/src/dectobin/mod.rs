
pub mod dectobin {

    pub fn exec(n:i32) -> String {
        let mut rez:String = String::new();
        let mut n = n;
        let mut r;

        while n > 1 {
            r = n % 2;
            n = n / 2;
            rez.push_str(r.to_string().as_str());
        }
        if n == 1 {
            rez.push_str("1");
        }
        rez.chars().rev().collect()
    }

    pub fn run() {
        println!("{} -> {}", 73, exec(72));
        println!("{} -> {}", 2, exec(2));
        println!("{} -> {}", 2222, exec(2222));
        println!("{} -> {}", 32895, exec(32895));
        println!("{} -> {}", 4953, exec(4953));
    }
}