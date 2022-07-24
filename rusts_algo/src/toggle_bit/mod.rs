
pub mod toggle_bit {

    pub fn exec(n:i32, k:u16) -> i32 {
        let m = 1 << k - 1;
        let r = n ^ m;
        println!("n - {:0>8b}({}), mask - {:0>8b}({})", n, n, m, m);
        println!("first toggle - {:0>8b}({})", r, r);
        let r2 = r ^ m;
        println!("second toggle - {:0>8b}({})", r2, r2);
        r
    }

    pub fn run() -> () {
        let n = 15;
        let m = 2;
        exec(n, m);
    }
    
}