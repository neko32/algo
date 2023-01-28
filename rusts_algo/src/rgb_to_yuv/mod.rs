
use ndarray::{Array1, Array2, arr1, arr2};
// note this conv is for ITU-R BT.709
pub fn exec(r:u32, g:u32, b:u32) -> (f32, f32, f32) {
    let r:f32 = r as f32;
    let g:f32 = g as f32;
    let b:f32 = b as f32;
    let rgb_arr:Array1<f32>  = arr1(&[r, g, b]);
    let conv_mat:Array2<f32> = arr2(&[
        [0.2999, 0.587, 0.114],
        [-0.168736, -0.331264, 0.5],
        [0.5, -0.418688, -0.081312]
    ]);
    let yuv_arr = conv_mat.dot(&rgb_arr);
    println!("RGB - {}, YUV ITU-R BT.709 - {}", rgb_arr.view(), yuv_arr.view());
    let rez = yuv_arr.as_slice().unwrap();
    (rez[0], rez[1], rez[2])
}

pub fn run() {
    let (r, g, b) = (255, 192, 128);
    exec(r, g, b);
}
