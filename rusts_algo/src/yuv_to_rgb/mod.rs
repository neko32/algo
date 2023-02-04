
use ndarray::{Array1, Array2, arr1, arr2};

// assume this conv is ITU-R BT.709
pub fn exec(y:f32, u:f32, v:f32) -> (u32, u32, u32) {
    let yuv_mat:Array1<f32> = arr1(&[y, u, v]);
    let conv_mat:Array2<f32> = arr2(&[
        [1., 0., 1.402],
        [1., -0.344136, -0.714136],
        [1., 1.772, 0.]
    ]);
    let rgb = conv_mat.dot(&yuv_mat);
    println!("YUV ITU-R BT.709 - {}, RGB - {}", yuv_mat.view(), rgb.view());
    let rez = rgb.as_slice().unwrap();
    (rez[0].trunc() as u32, rez[1].trunc() as u32, rez[2].trunc() as u32)
}

pub fn run() {
    let (y, u, v) = (203.7705, -42.630363, 36.70397);
    let rez = exec(y, u, v);
    println!("{:?}", rez);
}
