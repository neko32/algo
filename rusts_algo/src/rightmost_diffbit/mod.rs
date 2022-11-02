pub mod rightmost_diffbit {

    pub fn exec(m: u32, n: u32) -> u32 {
        println!("m:{:08b}({})", m, m);
        println!("n:{:08b}({})", n, n);
        let l = m ^ n;
        let r = !((m ^ n) - 1);
        println!("l:{:08b}({}), r:{:08b}({})", l, l, r, r);
        let rez = l & r;
        println!("l & r = {:08b}({})", rez, rez);
        rez
    }

    pub fn run() {
        let m = 11;
        let n = 13;
        println!("m:{},n:{} = {}", m, n, exec(m, n));
    }
}
