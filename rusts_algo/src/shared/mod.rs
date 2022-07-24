
pub mod shared {

    use std::default::Default;
    use derive_more::Display;

    #[derive(Display, Debug)]
    #[display(fmt = "{{x:{},y:{}}}", x, y)]
    pub struct Point {
        pub x:i32,
        pub y:i32,
    }

    impl Point {
        pub fn new(x_v:i32, y_v:i32) -> Self {
            Point {x:x_v, y:y_v}
        }
    }

    impl Default for Point {
        fn default() -> Self {
            Point::new(0, 0)    
        }
    }
}