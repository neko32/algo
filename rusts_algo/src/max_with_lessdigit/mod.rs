pub mod max_with_lessdigit {

    pub fn exec(n: u32) -> u32 {
        let s = n.to_string();
        let sb = s.as_bytes();
        let len = s.len();
        let mut maxv = std::u32::MIN;
        for i in 0..len {
            let x = String::from_utf8_lossy(&sb[0..i]).to_string();
            let y = String::from_utf8_lossy(&sb[i + 1..]).to_string();
            let a = x + &y;
            println!("{}", a);
            let av: u32 = a.parse().unwrap();
            maxv = maxv.max(av);
        }
        println!("n={},maxv={}", n, maxv);
        maxv
    }

    pub fn run() {
        let n = 1001;
        exec(n);
    }
}
