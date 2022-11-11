pub mod almost_increasing_seq {

    pub fn exec(s_orig: &[i32]) -> bool {
        let mut s = Vec::from_iter(s_orig.into_iter());
        let len = s.len();
        let mut penality = 0;
        for i in 1..len {
            println!("comparing {} and {}..", s[i - 1], s[i]);
            if s[i - 1] > s[i] {
                if penality >= 1 {
                    return false;
                } else {
                    println!("executing the only one-time swapping allowed..");
                    s.swap(i - 1, i);
                    penality += 1;
                }
            }
        }
        return true;
    }

    pub fn run() {
        let a = [1, 3, 2, 1];
        let r_a = exec(&a);
        let b = [1, 2, 3, 3, 5, 4, 6, 8];
        let r_b = exec(&b);
        println!("{:?} - {}, {:?} - {}", a, r_a, b, r_b);
    }
}
