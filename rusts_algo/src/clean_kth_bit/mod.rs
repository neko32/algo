pub mod clean_kth_bit {
    pub fn exec(v: u32, k: u8) -> u32 {
        println!("v = {:<016b}({})", v, v);
        let m = !(1 << k - 1);
        let r = v & m;
        println!("m = {:<016b}({})", m, m);
        println!("r = {:<016b}({})", r, r);
        r
    }

    pub fn run() {
        let n = 127;
        exec(n, 3);
    }
}
