
pub mod waterarea {

    pub fn exec(a:Vec<u32>) -> u32 {
        let mut reserved:u32 = 0;
        let len = a.len();
        let mut l = 0;
        let mut r = len - 1;
        let mut l_max = a[l];
        let mut r_max = a[r];
        while l < r {
            if a[l] < a[r] {
                l += 1;
                l_max = l_max.max(a[l]);
                let delta = l_max - a[l];
                reserved += delta;
                println!("adding {} from left side. Now reserved is {}", delta, reserved);
            } else {
                r -= 1;
                r_max = r_max.max(a[r]);
                let delta = r_max - a[r];
                reserved += delta;
                println!("adding {} from right side. Now reserved is {}", delta, reserved);
            }
        }
        reserved
    }

    pub fn run() -> () {
        let a:Vec<u32> = vec![0, 8, 0, 0, 5, 0, 0, 10, 0, 0, 1, 1, 0, 3];
        println!("waters = {:?}", a);
        println!("{}", exec(a));
    }

}