
pub fn exec(w:usize, h:usize, x:usize, y:usize, pix_size:usize) -> (usize, usize) {
    let stride_size = w * pix_size;
    let rez = stride_size * y + x * pix_size;
    println!("stride_size - {stride_size}, index st - {rez}, index end - {}", rez + pix_size - 1);
    (rez, rez + pix_size - 1)
}

pub fn run() {
    let (w, h) = (512, 512);
    let (x, y) = (120, 247);
    let pix_size = 4;
    exec(w, h, x, y, pix_size);
}
