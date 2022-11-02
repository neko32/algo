pub mod even_num_sum {

    pub fn exec(n: u32) -> bool {
        let ns: String = n.to_string();
        let bn = ns.as_bytes();
        let len = bn.len();
        let (former, latter) = bn.split_at(len / 2);
        let fs: u32 = former
            .iter()
            .cloned()
            .map(|b| (b as char).to_digit(10).unwrap())
            .sum();
        let ls: u32 = latter
            .iter()
            .cloned()
            .map(|b| (b as char).to_digit(10).unwrap())
            .sum();
        let rez = fs == ls;
        println!("{} - {} - {} - {}", n, fs, ls, rez);
        rez
    }

    pub fn run() {
        let a = 124568;
        let b = 1230;
        exec(a);
        exec(b);
    }
}
