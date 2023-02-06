
use regex::Regex;
pub fn exec<'a>(s:&'a str) -> Result<String, anyhow::Error> {
    let regex = Regex::new(r"^(<[\w]*)( *>?)(.*$)").unwrap();
    let maybe_capts = regex.captures(s);
    if maybe_capts.is_none() {
        return Err(anyhow::anyhow!("invalid input"));
    }
    let capts = maybe_capts.unwrap();
    match capts.get(1) {
        Some(c) => {
            let mut buf = c.as_str().to_string();
            buf.insert(1, '/');
            if !buf.ends_with(">") {
                buf.push_str(">");
            }
            println!("input - {s}, result - {buf}");
            Ok(buf)
        },
        None => {
            Err(anyhow::anyhow!("failed to build closure tag for {s}"))
        }
    }
}

pub fn run() {
    exec("<button type='tako' disabled>neko desu.").unwrap();
    exec("<button>neko desu.").unwrap();
    exec("<button  >neko desu.").unwrap();
}
