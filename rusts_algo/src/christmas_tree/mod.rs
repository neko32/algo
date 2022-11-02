pub mod christmas_tree {

    pub fn exec(t: &[&str]) -> Vec<String> {
        let len = t[0].len();
        let mut buf: Vec<String> = Vec::new();
        let mut head: String = String::new();
        for _ in 0..=len {
            head.push_str("-");
        }
        buf.push(head);
        for l in t {
            let mut cnt: i32 = l.chars().map(|x| if x == '*' { 1 } else { 0 }).sum();
            let m = len as i32 - cnt;
            let space_num = m / 2;
            let mut s: String = String::new();
            if cnt == len as i32 {
                cnt -= 1;
            }
            s.push_str("|");
            for _ in 0..space_num {
                s.push_str(" ");
            }
            for _ in 0..cnt {
                s.push_str("*");
            }
            for _ in 0..space_num {
                s.push_str(" ");
            }
            s.push_str("|");
            buf.push(s);
        }

        let mut foot: String = String::new();
        for _ in 0..=len {
            foot.push_str("-");
        }
        buf.push(foot);

        for l in &buf {
            println!("{}", l);
        }

        buf
    }

    pub fn run() {
        let t = [
            "       *  ",
            "     *    ",
            "***       ",
            "     *****",
            "   *******",
            "**********",
            " ***      ",
        ];
        exec(&t);
    }
}
