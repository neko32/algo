pub mod array_product_sum {

    pub fn exec(a: &[i32]) -> Vec<i32> {
        let len = a.len();
        let mut l: Vec<i32> = std::iter::repeat(1).take(len).collect();
        let mut r: Vec<i32> = std::iter::repeat(1).take(len).collect();
        let mut rez: Vec<i32> = std::iter::repeat(1).take(len).collect();

        let mut t = 1;
        for i in 1..=len {
            l[i - 1] = t;
            t *= a[i - 1];
            println!("{},{}", a[i - 1], t);
        }
        println!("{:?}", l);
        t = 1;
        for i in (0..=len - 2).rev() {
            r[i + 1] = t;
            t *= a[i + 1];
            println!("{},{}", a[i + 1], t);
        }
        println!("{},{}", a[0], t);
        r[0] = t;
        println!("{:?}", r);
        for i in 0..len {
            rez[i] = l[i] * r[i];
        }

        rez
    }

    pub fn run() {
        let a = [5, 1, 4, 2];
        let p = &a[..];
        let r = exec(p);
        println!("input = {:?}, output = {:?}", p, r);
    }
}
