pub mod helon_formula {
    pub fn exec(a: f32, b: f32, c: f32) -> f32 {
        let s = (a + b + c) / 2_f32;
        let a = (s * (s - a) * (s - b) * (s - c)).sqrt();
        println!("s = {}, area = {}", s, a);
        return a;
    }

    pub fn run() {
        let a = 5.0;
        let b = 6.0;
        let c = 7.0;
        exec(a, b, c);
    }
}
