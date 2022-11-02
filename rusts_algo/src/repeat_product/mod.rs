pub mod repeat_product {

    use std::collections::HashSet;

    pub fn exec(v: u32) -> u32 {
        let mut n: i32 = v as i32;
        let mut s: HashSet<i32> = HashSet::from_iter([n]);
        let mut seqn = 1;
        'lp: loop {
            seqn += 1;
            let sa = n.to_string();
            n = sa
                .as_bytes()
                .iter()
                .map(|ch| {
                    let c = ch - 48;
                    c.pow(2) as i32
                })
                .sum();
            if s.contains(&n) {
                break 'lp;
            } else {
                s.insert(n);
            }
            println!("{}:{:?}", seqn, s);
        }
        println!("{}", seqn);
        seqn
    }

    pub fn run() -> () {
        println!("n = 16");
        println!("{}", exec(16));
    }
}
