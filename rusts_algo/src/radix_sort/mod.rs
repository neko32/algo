
pub mod radix_sort {

    pub fn exec(v:&mut Vec<i32>) -> () {
        let mut d = 0;
        let mut maxv = *v.iter().max().unwrap();
        while maxv > 0 {
            radix(v, d);
            d += 1; 
            maxv /= 10;
        }
    }

    fn radix(v:&mut Vec<i32>, d:u32) -> () {
        let mut sorted = Vec::from_iter(std::iter::repeat(0).take(v.len()));
        let mut count = Vec::from_iter(std::iter::repeat(0).take(10));
        let len = v.len();
        for i in 0..len {
            let x = (v[i] / 10_i32.pow(d)) % 10;
            count[x as usize] += 1;
        }
        for i in 1..len {
            count[i] += count[i - 1];
        }
        for i in (0..len).rev() {
            let x = (v[i] / 10_i32.pow(d)) % 10;
            count[x as usize] -= 1;
            let idx = count[x as usize];
            sorted[idx] = v[i];
        }
        for i in 0..len {
            v[i] = sorted[i];
        }
    }

    pub fn run() -> () {
        let mut v = vec![8762, 654, 3008, 345, 87, 65, 234, 12, 2];
        exec(&mut v);
        println!("{:?}", v);
    }
}