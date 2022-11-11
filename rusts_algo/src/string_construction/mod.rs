pub mod string_construction {

    use std::collections::HashMap;

    pub fn exec(a: &str, b: &str) -> u32 {
        let alen = a.len();
        let blen = b.len();
        let ab = a.as_bytes();
        let bb = b.as_bytes();
        let mut m: HashMap<char, u32> = HashMap::new();
        let mut cnt: u32 = 0;
        let mut remaining = blen;
        for i in 0..blen {
            let ch = bb[i] as char;
            let p = m.entry(ch).or_insert(0);
            *p += 1;
        }

        'lp: while remaining >= alen {
            let mut completed = true;
            'inner: for i in 0..alen {
                let ch = ab[i] as char;
                let mut v = match m.get(&ch) {
                    Some(x) => *x,
                    None => 0,
                };
                if v == 0 {
                    completed = false;
                    break 'inner;
                } else {
                    remaining -= 1;
                    v += 1;
                    m.insert(ch, v);
                }
            }
            if completed {
                cnt += 1;
            } else {
                break 'lp;
            }
        }
        cnt
    }

    pub fn run() -> () {
        let a = "abc";
        let b = "abccba";
        let rez = exec(a, b);
        println!("a={},b={},rez={}", a, b, rez);
    }
}
