
use std::collections::HashMap;

pub fn exec<'a>(current_state:&'a str) -> Result<&'a str, anyhow::Error> {
    let h:HashMap<&'a str, &'a str> = hmap::hmap!(
        "start" => "in_game",
        "in_game" => "end"
    );
    match h.get(current_state) {
        Some(r) => {
            println!("{current_state} -> {r}");
            Ok(r)
        },
        None => {
            println!("no transition found");
            Err(anyhow::anyhow!("no transition"))
        }
    }
}

pub fn run() {
    let init = "start";
    let a = exec(init).unwrap();
    let b = exec(a).unwrap();
    match exec(b) {
        Ok(_) => panic!("not expected"),
        Err(e) => println!("{}", e.downcast::<&str>().unwrap()),
    }
}
