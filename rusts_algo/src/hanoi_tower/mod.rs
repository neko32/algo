
pub fn exec(n:u32) -> Vec<String> {
    let mut buf:Vec<String> = Vec::new();
    mov(n, 'A', 'B', 'C', &mut buf);
    buf
}

fn mov(depth:u32, src:char, dest:char, aside:char, buf:&mut Vec<String>) {
    match depth {
        0 => (),
        _ => {
            mov(depth - 1, src, aside, dest, buf);
            buf.push(format!("moved a ring {} at {} to {} via {}", depth, src, dest, aside));
            mov(depth - 1, aside, dest, src, buf);
        }
    }
}

pub fn run() {
    let rez = exec(4);
    rez.iter().for_each(|s|println!("{}", s));
}