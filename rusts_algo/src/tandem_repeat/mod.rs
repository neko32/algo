

pub mod tandem_repeat {

    pub fn exec(s:String) -> bool {
        let len = s.len();
        if len % 2 == 1 {
            return false;
        }
        let (a,b) = s.split_at(len / 2);
        let ab = a.as_bytes();
        let bb = b.as_bytes();
        let slen = ab.len();
        for i in 0..slen {
            if ab[i] != bb[i] {
                return false;
            }
        }
        return true;
    }

    pub fn run() {
        let s = "nekoneko".to_string();
        println!("{}", exec(s));
    }
}