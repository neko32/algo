
pub fn exec(c1:(u8, u8, u8), c2:(u8, u8, u8)) -> bool {
    let r1 = c1.0 as u32;
    let g1 = c1.1 as u32;
    let b1 = c1.2 as u32;
    let r2 = c2.0 as u32;
    let g2 = c2.1 as u32;
    let b2 = c2.2 as u32;
    // brightness diff test formula
    // abs((299 * R1 + 587 * G1 + 114 * B1) / 1000) - (299 * R2 + 587 * G2 + 114 * B2)) > 125
    // hue diff test formula
    // abs(R1 - R2) + abs(G1 - G2) + abs(B1 - B2) > 500
    let bd1 = (r1 * 299 + g1 * 587 + b1 * 114) / 1000;
    let bd2 = (r2 * 299 + g2 * 587 + b2 * 114) / 1000;
    let brightness_diff_high = bd1.abs_diff(bd2);
    let hue_diff_high = r1.abs_diff(r2) + g1.abs_diff(g2) + b1.abs_diff(b2);
    let rez = brightness_diff_high > 125 && hue_diff_high > 500;
    println!("brightness diff high? (>125 high) - {brightness_diff_high}");
    println!("hue diff high? (>500 high) - {hue_diff_high}");
    println!("result - {rez}");
    rez
}

pub fn run() {
    exec((60, 30, 20), (180, 200, 255));
    exec((100, 200, 220), (180, 200, 255));
}
