
pub fn exec(s:&str) -> &str {
    let len = s.len();
    let mut longest_pair:(usize, usize) = (0, 0);
    for i in 0..len {
        let i:i32 = i as i32;
        let even = find(s, i - 1, i);
        let odd = find(s, i - 1, i + 1);
        print!("even[{}] vs odd[{}] - ", construct(s, even), construct(s, odd));
        let bigger = {
            if even.1 - even.0 > odd.1 - odd.0 {
                even
            } else {
                odd
            }
        };
        println!("bigger[{}] vs longest_pair[{}]", construct(s, bigger), construct(s, longest_pair));
        if (bigger.1 - bigger.0) > (longest_pair.1 - longest_pair.0) {
            longest_pair = bigger;
        }
    }
    &s[longest_pair.0..longest_pair.1]
}

fn construct(s:&str, p:(usize, usize)) -> &str {
    &s[p.0..p.1]
}

fn find(s:&str, l:i32, r:i32) -> (usize, usize) {
    let mut l = l;
    let mut r = r;
    let sb = s.as_bytes();
    while l >= 0 && r < s.len() as i32 {
        println!("l={},r={},s[l]={},s[r]={}", l, r, sb[l as usize] as char, sb[r as usize] as char);
        if sb[l as usize] as char != sb[r as usize] as char {
            break;
        }
        l -= 1;
        r += 1;
    }
    ((l + 1) as usize, r as usize)
}

pub fn run() {
    let s = "abaxyzzyxf";
    println!("input - {}, longest pelindromic substr - {}", s, exec(s));
}
