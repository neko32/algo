
pub mod least_data_eviction {

    pub fn exec(seq:&Vec<i32>) -> Vec<i32> {
        println!("input - {:?}", &seq);
        let mut v:Vec<i32> = Vec::new();
        seq.iter().for_each(|n|{ins(*n, &mut v)});
        println!("output - {:?}", &v);
        v
    }

    fn ins(n:i32, v:&mut Vec<i32>) {
        let len = v.len();
        let mut idx = 0;
        while idx < len && v[idx] < n {
            idx += 1;
        }
        v.insert(idx, n);
        if v.len() > 3 {
            v.remove(0);
        }
    }

    pub fn run() {
        let s = vec![8, 2, 12, 5, 22, 3, 4, 9, 10, 2, 25, 3, 16];
        exec(&s);
    }

}