

pub fn exec<T, N>(v:&[T]) -> (N, N)
where T: Eq + PartialEq + PartialOrd + MinMaxVal<N> + Copy,
N: Eq + PartialEq + PartialOrd {
    find(v)
}

fn find<T, N>(v:&[T]) -> (N, N)
where T: Eq + PartialEq + PartialOrd + MinMaxVal<N> + Copy,
N: Eq + PartialEq + PartialOrd {
    let mut greatest:N = T::min_val();
    let mut least:N = T::max_val();
    for x in v {
        if x.val() > greatest {
            greatest = x.val();
        } else if x.val() < least {
            least = x.val();
        }
    }
    (least, greatest)
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct IntNum {
    pub n:i32
}

impl IntNum {
    pub fn new(v:i32) -> Self {
        IntNum {n:v}
    }
}

impl MinMaxVal<i32> for IntNum {
    fn min_val() -> i32 {
        i32::MIN
    }
    fn max_val() -> i32 {
        i32::MAX
    }
    fn val(&self) -> i32 {
        self.n
    }
}

pub trait MinMaxVal<T> {
    fn min_val() -> T;
    fn max_val() -> T;
    fn val(&self) -> T;
}

pub fn run() {
    let v:Vec<IntNum> = vec![IntNum::new(8), IntNum::new(12), IntNum::new(25), IntNum::new(3), IntNum::new(50)];
    let (least, greatest) = exec(&v);
    println!("input - {:?}", v);
    println!("least - {}, greatest - {}", least, greatest);
}