
pub fn exec(v:Vec<Pair>) -> Vec<Pair> {
    let mut buf:Vec<Pair> = Vec::new();

    let mut idx = 0;
    let len = v.len();
    while idx < len {
        let st = v[idx].st;
        let mut end = v[idx].end;
        while idx + 1 < len && v[idx + 1].st < end {
            idx += 1;
            end = end.max(v[idx].end);
        }
        buf.push(Pair::from((st, end)));
        idx += 1;
    }
    println!("input - {:?}", v);
    println!("result - {:?}", buf);
    buf
}


#[derive(Debug, Eq, PartialEq)]
pub struct Pair {
    pub st:u32,
    pub end:u32,
}

impl Pair {
    pub fn new(st:u32, end:u32) -> Self {
        Self { st:st, end:end }
    }
}

impl From<(u32, u32)> for Pair {
    fn from(value: (u32, u32)) -> Self {
        Self {st: value.0, end: value.1 }
    }
}

pub fn run() {
    let n:Vec<Pair> = vec![Pair::new(1, 2), Pair::new(3, 5), Pair::new(4, 7),
    Pair::new(6, 8), Pair::new(9, 10)];
    exec(n);
}