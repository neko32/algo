pub mod two_sum {

    use std::collections::HashMap;

    pub fn exec(n: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h: HashMap<i32, i32> = HashMap::new();
        let len = n.len();
        for i in 0..len {
            let x = target - n[i];
            if h.contains_key(&x) {
                return Vec::from_iter([n[i], x]);
            }
            h.insert(n[i], i as i32);
        }
        Vec::new()
    }

    pub fn run() {
        assert!(exec(vec![3, 5, -4, 8, 11, 1, -1, 6], 10) == vec![-1, 11])
    }
}
