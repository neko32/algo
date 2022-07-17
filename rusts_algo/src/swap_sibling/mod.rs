
pub mod swap_sibling {

    pub fn exec<T>(v:&mut Vec<T>) -> () {
        let siz = v.len();
        for i in 0..siz / 2 {
            v.swap(i * 2, i * 2 + 1);
        }
    }

    pub fn run() -> () {
        let mut v:Vec<i32> = (1..=6).collect();
        println!("{:?}", v);
        exec(&mut v);
        println!("{:?}", v);
    }
}