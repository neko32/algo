

pub fn exec(pixel:&[u8;3]) -> [u8;3] {
    // rgb
    // gbr
    [pixel[2], pixel[1], pixel[0]]
}

pub fn run() {
    let magenta = [255, 0, 255];
    println!("RGB - {:?} - BGR - {:?}", magenta, exec(&magenta));
}
