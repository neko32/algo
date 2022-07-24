
pub mod smallest_positive_product {

    pub fn exec(n:u32) -> i32 {
        let mut mn = n;
        let mut v:Vec<u32> = Vec::new();

        if mn == 1 {
            return 1;
        } else if mn == 0 {
            return 10;
        }

        while mn > 1 {
            let mut cont = false;
            'inner: for i in (2..=9).rev() {
                if mn % i == 0 {
                    v.push(i);
                    mn /= i;
                    cont = true;
                    break 'inner;
                }
            }
            if !cont {
                return -1;
            }
        }

        v.sort_by(|a,b|{a.cmp(&b)});
        let x:String = v.iter().map(|x|{x.to_string()}).collect();
        x.parse::<i32>().unwrap()

    }

    pub fn run() {
        println!("{}", exec(12));
        println!("{}", exec(19));
    }
}