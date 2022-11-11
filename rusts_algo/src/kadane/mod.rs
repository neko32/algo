pub mod kadane {

    use std::cmp::max;

    pub fn exec(v: Vec<i32>) -> i32 {
        let mut grand_max_so_far = v[0];
        let mut max_so_far = v[0];
        for i in 1..v.len() {
            max_so_far = max(v[i], max_so_far + v[i]);
            grand_max_so_far = max(grand_max_so_far, max_so_far);
        }
        grand_max_so_far
    }

    pub fn run() {
        let v = vec![3, 5, -9, 1, 3, -2, 3, 4, 7, 2, -9, 6, 3, 1, -5, 4];
        println!("{}", exec(v));
    }
}
