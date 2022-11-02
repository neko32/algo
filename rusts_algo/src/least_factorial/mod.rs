pub mod least_factorial {
    pub fn exec(n: u32) -> u32 {
        let mut x = 2_u32;
        while *factorial(Box::new(x)) < n {
            x += 1;
        }
        let rez = *factorial(Box::new(x));
        println!("n={}, rez={}", n, rez);
        rez
    }

    pub fn run() {
        let n = 17;
        exec(n);
    }

    fn factorial(n: Box<u32>) -> Box<u32> {
        if *n == 1 {
            Box::new(1)
        } else {
            Box::new(*n * *factorial(Box::new(*n - 1)))
        }
    }
}
