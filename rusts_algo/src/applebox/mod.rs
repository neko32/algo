
pub mod applebox {

    pub fn exec(n:u8) -> i32 {
        let mut red:i32 = 0;
        let mut yellow:i32 = 1;
        for i in 2..=n {
            if i % 2 == 0 {
                red += (i * i) as i32; 
            } else {
                yellow += (i * i) as i32;
            }
        }
        let rez = red - yellow;
        println!("n = {}", n);
        println!("yellow = {}, red = {}, red - yellow = {}", yellow, red, rez);
        rez
    }

    pub fn run() {
        exec(5);
    }
}