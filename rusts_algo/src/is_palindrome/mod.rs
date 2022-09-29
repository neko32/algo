
pub mod is_palindrome {

    pub fn exec(s:&String) -> bool {
        let mut l = 0;
        let mut r = s.len() - 1;
        let sb = s.as_bytes();
        while l < r {
            if sb[l] as char != sb[r] as char {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true  
    }

    pub fn run() -> () {
        let s = "davvad".to_string();
        let a = "abcba".to_string();
        let b = "abaya".to_string();
        println!("{}={},{}={},{}={}", &s, exec(&s), &a, exec(&a), &b, exec(&b)); 
    }
}