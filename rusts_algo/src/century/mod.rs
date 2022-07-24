
pub mod century {
    pub fn exec(n:u32) -> u32 {
        let a = n / 100;
        let b = n % 100;
        if b != 0 {
            a + 1
        } else {
            a
        }
    }

    pub fn run() -> () {
        let a = 1905;
        let ar = exec(a);
        let b = 2000;
        let br = exec(b);
        println!("{}=>{},{}=>{}", a, ar, b, br);
    }
}