
pub fn exec(cmd:&str) -> u32 {
    let mut front_facing = 0_u32;
    let cmd = cmd.to_ascii_uppercase();
    let mut ctr = 0;

    for c in cmd.chars() {
        match c {
            'L' | 'R' => ctr += 1,
            'A' => ctr += 2,
            _ => (),
        };
        if ctr % 2 == 0 {
            front_facing += 1;
        }
    }

    println!("commands - {}, result - {}", cmd, front_facing);
    front_facing
}

pub fn run() {
    let cmd = "LLARL";
    exec(cmd);
}