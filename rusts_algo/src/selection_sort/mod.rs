
pub mod selection_sort {


    pub fn exec(v:&mut Vec<i32>) -> () {
        let len = v.len();

        for i in 0..len - 1 {
            let mut minv = v[i];
            let mut midx = i;
            for j in i + 1..len {
                if v[j] < minv {
                    minv = v[j];
                    midx = j;
                }
            }
            v.swap(i, midx);
        }
    }

    pub fn run() {
        let mut v = vec![10, 7, 55, 42, 1, 9, -6, 34, 8, 29, 7];
        exec(&mut v);
        println!("{:?}", v);
    }
}