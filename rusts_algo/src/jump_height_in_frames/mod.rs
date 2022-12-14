
pub fn exec(frame:u32, num_of_frames:u32, height:u32) -> f32 {
    let x = frame as f32 * 180_f32 / num_of_frames as f32;
    let r = (x as f32).to_radians();
    let s = r.sin();
    let h = s * height as f32;
    println!("{}/{} with max height {}", frame, num_of_frames, height);
    println!("x - {}, rad - {}, sin({}) - {}, height - {}", x, r, r, s, h);
    h
}

pub fn run() {
    exec(0, 40, 120);
    exec(20, 40, 120);
    exec(40, 40, 120);
}