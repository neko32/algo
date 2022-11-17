
use std::collections::HashMap;

pub fn exec(h:i32) -> String {
    let mut buf:HashMap<(i32, i32), i32> = HashMap::new();
    buf.insert((1, 1), 1);

    for n in 2..=h {
        for r in 1..=n {
            if r == 1 || r == n {
                buf.insert((n, r), 1);
            } else {
                let a = buf.get(&(n - 1, r - 1)).unwrap();
                let b = buf.get(&(n - 1, r)).unwrap();
                buf.insert((n, r), a + b);
            }
        }
    }

    let mut s:String = String::new();
    for r in 1..=h {
        let mut rows:Vec<String> = Vec::new();
        for c in 1..=r {
            let v = buf.get(&(r, c)).unwrap();
            rows.push(format!("{}", v));
        }
        s.push_str(rows.join(" ").as_str());
        s.push_str("\n");
    }

    println!("{}", &s);
    s

}

pub fn run() {
    let h = 5;
    exec(h);
}
