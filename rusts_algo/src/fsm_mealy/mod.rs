
use std::collections::HashMap;
use hmap::hmap;
use anyhow::anyhow;
pub fn exec<'a>(current_state:&'a str, input:&'a str) -> Result<&'a str, anyhow::Error> {
    let h:HashMap<(&str, &str), &str> = hmap!(
        ("stop", "start") => "running",
        ("running", "stop") => "stop",
        ("running", "pause") => "pause",
        ("pause", "start") => "running"
    );
    match h.get(&(current_state, input)) {
        Some(r) => {
            println!("({},{}) -> {}", current_state, input, r);
            Ok(r)
        },
        None => {
            println!("({},{}) -> NA", current_state, input);
            Err(anyhow!("not supported transition"))
        }
    }
}

pub fn run() {
    let init = "stop";
    let a = exec(init, "start").unwrap();
    let b = exec(a, "pause").unwrap();
    let c = exec(b, "start").unwrap();
    let d = exec(c, "stop").unwrap();
    let e = exec(d, "stop");
    match e {
        Ok(_) => println!("??"),
        Err(e) => println!("{}", e.downcast::<&str>().unwrap()),
    }
}
