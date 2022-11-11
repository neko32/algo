pub mod sigma_k {

    pub fn exec(k: i32) -> i32 {
        (k * (k + 1)) / 2
    }

    pub fn run() {
        println!("k = 7, sigma_k = {}", exec(7));
    }
}
