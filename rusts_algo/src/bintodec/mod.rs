pub mod bin_to_dec {

    pub fn exec(b: &str) -> i32 {
        let bs = b.as_bytes();
        let mut idx = (bs.len() - 1) as i32;
        let mut p = 1;
        let mut sumv = 0;
        while idx >= 0 {
            if bs[idx as usize] as char == '1' {
                sumv += p;
            }
            p <<= 1;
            idx -= 1;
        }
        sumv
    }

    pub fn run() -> () {
        let b = "10001010";
        println!("{}", exec(b));
    }
}
