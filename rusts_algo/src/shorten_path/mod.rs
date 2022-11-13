
pub fn exec(path:&str) -> String {

    let bs = path.as_bytes();
    let mut idx = 0;
    let mut rez:Vec<String> = Vec::new();
    let len = bs.len();

    let is_absolute = if bs[0] as char == '/' {
        idx = 1;
        rez.push("".to_string());
        true
    } else {
        false
    };

    while idx < len {
        let mut k = idx + 1;
        while k < len && bs[k] as char != '/' && bs[idx] as char != '/' {
            k += 1;
        }
        let w = String::from_utf8_lossy(&bs[idx..k]).to_string();
        print!("working on \"{}\" | ", &w);

        match w {
            s if s.is_empty() || s == "." => {
                print!("action - NOOP | ");
            },
            s if s == "/" => {
                idx = k;
                print!("action - NOOP | ");
                continue;
            }
            s if s == ".." && rez.is_empty() && is_absolute => {
                print!("action - append root | ");
                rez.push("".to_string())
            },
            s if s == ".." && rez.last().unwrap_or(&"NA".to_string()) == ".." => {
                print!("action - append .. | ");
                rez.push("..".to_string())
            },
            s if s == ".."  => {
                print!("pop previous item .. | ");
                rez.pop();
                if rez.is_empty() && is_absolute {
                    rez.push("".to_string());
                }
            },
            s => {
                print!("action - append {} | ", &s);
                rez.push(s)
            },
        };
        idx = k + 1;
        println!("buf - {:?}({})", &rez, rez.len());
    }

    let mut rs = rez.join("/");
    if rs.is_empty() {
        rs.push_str("/");
    }
    println!("original path - {}, shortened path - {}", path, &rs);
    rs
}

pub fn run() {
    let path = "/foo/../test/../test/../foo//bar/./baz";
    exec(path);
}