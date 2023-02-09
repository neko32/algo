
pub fn exec(attempt:&str, ans:&str) -> String {

    let b_attempt = attempt.as_bytes();
    let b_ans = ans.as_bytes();
    let mut rez:String = String::new();

    for (attempt_idx, attempt_b) in b_attempt.iter().enumerate() {
        let attempt_c = *attempt_b as char;
        let mut blow = false;
        let mut hit = false;
        'inner: for (ans_idx, ans_c) in b_ans.iter().enumerate() {
            let ans_c = *ans_c as char;
            if attempt_idx == ans_idx && attempt_c == ans_c {
                rez.push_str("H");
                hit = true;
                break 'inner;
            } else if attempt_c == ans_c {
                blow = true;
                break 'inner;
            }
        }
        if !hit && blow {
            rez.push_str("B");
        } else if !hit && !blow {
            rez.push_str("_");
        }
    }
    println!("attempt - {attempt}, ans - {ans}, hint - {rez}");
    rez
}

pub fn run() {
    let ans = "708";
    exec("212", ans);
    exec("549", ans);
    exec("756", ans);
    exec("780", ans);
    exec("708", ans);
}
