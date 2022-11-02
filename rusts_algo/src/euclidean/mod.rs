pub mod euclidean {

    pub fn exec(m: u32, n: u32) -> u32 {
        let mut m = m;
        let mut n = n;
        let mut o: u32 = 0;
        println!("[BEGIN] m={},n={}", m, n);
        while n != 0 {
            o = m % n;
            m = n;
            n = o;
            println!("m={},n={}", m, n);
        }
        println!("[END] m={},n={}", m, n);
        m
    }

    pub fn run() {
        let m = 128;
        let n = 72;
        println!("{}", exec(m, n));
    }
}
