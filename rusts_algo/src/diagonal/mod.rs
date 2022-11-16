pub mod diagonal {

    use crate::shared::Point;

    pub fn exec(a: &Point, b: &Point) -> bool {
        a.x + a.y == b.x + b.y || a.x - a.y == b.x - b.y
    }

    pub fn run() -> () {
        let a = Point::new(1, 5);
        let b = Point::new(9, 13);
        let c = Point::new(3, 0);
        let d = Point::new(6, 0);
        println!(
            "a:{},b:{} must be diagonal - result is {}",
            a,
            b,
            exec(&a, &b)
        );
        println!(
            "c:{},d:{} must NOT be diagonal - result is {}",
            c,
            d,
            exec(&c, &d)
        );
    }
}
