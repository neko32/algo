

pub mod binary_search {

    pub fn exec(s:&Box<Vec<i32>>, t:i32, l:usize, r:usize) -> i32 {
        println!("l={},r={}", l, r);
        if l > r {
            return -1;
        }
        let m = (l + r) / 2;
        if s[m] == t {
            m as i32
        } else if t < s[m] {
            exec(s, t, l, m - 1)
        } else {
            exec(s, t, m + 1, r)
        }
    }

    pub fn run() {
        let s = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
        let len = s.len();
        let b = Box::new(s);
        println!("vec - {:?}", &b);
        println!("{}", exec(&b, 33, 0, len - 1));
    }
}