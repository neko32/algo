
pub mod add_two_without_carry {

    use std::cmp::max;

    pub fn exec(a:u32, b:u32) -> u32 {
        let mut d = 0;
        let mut maxv = max(a, b);
        let mut buf:Vec<String> = Vec::new();
        while maxv > 0 {
            let x = a / 10_u32.pow(d) % 10;
            let y = b / 10_u32.pow(d) % 10;
            println!("{} + {} = {}", x, y, (x + y) % 10);
            buf.push(((x + y) % 10).to_string());
            maxv /= 10;
            d += 1;
        }
        buf.reverse();
        let rez = u32::from_str_radix(buf.concat().as_str(), 10).unwrap();
        println!("a - {}, b - {}, rez - {}", a, b, rez);
        rez
    }

    pub fn run() {
        let a = 456;
        let b = 1180;
        exec(a, b);
    }
}