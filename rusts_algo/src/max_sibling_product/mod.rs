pub mod max_sibling_product {

    pub fn exec(v: Vec<i32>) -> i32 {
        let mut m = v[0];
        let mut n = v[0] * v[1];
        let mut maxv = m.max(n);
        let mut idx = 2;
        let len = v.len();

        while idx < len {
            m = n;
            n = v[idx] * v[idx - 1];
            maxv = m.max(n);
            println!("m={},n={}, then maxv={}", m, n, maxv);
            idx += 1;
        }

        maxv
    }

    pub fn run() -> () {
        let v = vec![3, 6, -2, -5, 7, 3];
        println!("{}", exec(v));
    }
}
