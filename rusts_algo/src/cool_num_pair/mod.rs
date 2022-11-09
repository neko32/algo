

pub mod cool_num_pair {

    use std::collections::HashMap;
    pub fn exec(a:&[i32], b:&[i32]) -> u32 {

        let mut h:HashMap<i32, i32> = HashMap::new();
        let alen = a.len();
        let blen = b.len();

        let chk = |x,y|{
            if (x * y) % (x + y) == 0 {
                Some(x + y)
            } else {
                None
            }
        };

        for i in 0..alen {
                let x = a[i];
                for j in 0..blen {
                    let y = b[j];
                    println!("{},{}", x, y);
                    let Some(r) = chk(x, y) else {
                        continue
                    };
                    println!("r - {}", r);
                    h.insert(r, r);
            }
        }

        println!("{:?}", h);
        h.len() as u32

    }

    pub fn run() {
        let a = [4, 5, 6, 7, 8];
        let b = [8, 9, 10, 11, 12];
        println!("a - {:?}, b - {:?}", a, b);
        println!("{}", exec(&a, &b));
    }

}