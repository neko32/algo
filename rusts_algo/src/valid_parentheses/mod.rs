
pub fn exec(s:&str) -> bool {
    let cs = s.chars();
    let mut stack:Vec<char> = Vec::new();
    for c in cs {
        match c {
            '{' | '(' | '[' => stack.push(c),
            ']' => {
                let p = stack.pop();
                if let Some(cc) = p {
                    if cc != '[' {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            '}' => {
                let p = stack.pop();
                if let Some(cc) = p {
                    if cc != '{' {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            ')' => {
                let p = stack.pop();
                if let Some(cc) = p {
                    if cc != '(' {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            _ => (),
        }
    }

    stack.is_empty()
}

pub fn run() {
    println!("{}", exec("{[[((()))]]}"));
    println!("{}", exec("{[neko]}"));
    println!("{}", exec("(([))"));
    println!("{}", exec("[[]"));
    println!("{}", exec("[]]"))
}
