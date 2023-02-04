
use crate::shared::FPoint;

pub fn exec(points:&Vec<FPoint>, t_start:f32, t_end:f32, scale:f32) -> Vec<FPoint> {
    let mut interpolated:Vec<FPoint> = Vec::new();
    let mut t = t_start;

    while t < t_end {
        let ip_y = calc(points, t);
        interpolated.push(FPoint::new(t, ip_y));
        t += scale;
    }
    println!("input - {:?}", points);
    println!("result - {:?}", interpolated);
    interpolated
}

pub fn calc(points:&Vec<FPoint>, t:f32) -> f32 {
    let n = points.len();
    let mut sumv:f32 = 0.;
    for i in 0..n {
        let mut yv = points[i].y;
        for j in 0.. n {
            if i != j {
                yv = yv * ((t - points[j].x) / (points[i].x - points[j].x));
            }
        }
        sumv += yv;
    }
    sumv
}

pub fn run() {
    let points = vec![FPoint::new(0.0, 0.8),
    FPoint::new(1.0, 3.1),
    FPoint::new(3.0, 4.5),
    FPoint::new(6.0, 3.9),
    FPoint::new(7.0, 2.8)];
    exec(&points, 0_f32, 7.1_f32, 0.5_f32);
}