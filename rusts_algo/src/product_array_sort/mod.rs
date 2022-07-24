pub mod product_array_sort {

    pub fn exec(v:&Vec<i32>) -> Vec<i32> {
        let mut w:Vec<i32> = Vec::from_iter(std::iter::repeat(0).take(v.len()));
        let mut l = 0;
        let mut r = (v.len() - 1) as i32;
        let mut idx = (v.len() - 1) as i32;
        
        while idx as i32 >= 0 {
            if v[l].abs() < v[r as usize].abs() {
                w[idx as usize] = v[r as usize].pow(2);
                r -= 1;
            } else {
                w[idx as usize] = v[l].pow(2);
                l += 1;
            }
            idx -= 1;
        }
        w
    }

    pub fn run() -> () {
        let v:Vec<i32> = vec![-11, -6, 0, 5, 8, 10];
        let r = exec(&v);
        println!("orig - {:?}", v);
        println!("after - {:?}", r);
    }
}