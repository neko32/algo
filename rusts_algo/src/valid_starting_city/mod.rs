
pub fn exec(distance:Vec<i32>, fuel:Vec<i32>, mpg:i32) -> usize {

    println!("distance - {:?}, fuel - {:?}, mpg - {mpg}", distance, fuel);
    let len = distance.len();
    let mut valid_start_city:usize = 0;
    let mut min_val_so_far = 0;
    let mut remain = 0;
    for i in 1..len {
        remain += fuel[i - 1] * mpg - distance[i - 1];
        if remain < min_val_so_far {
            min_val_so_far = remain;
            valid_start_city = i;
        }
    }
    println!("valid start city is {valid_start_city}");
    valid_start_city
}

pub fn run() {
    let dist = vec![5, 25, 15, 10, 15];
    let fuel = vec![1, 2, 1, 0, 3];
    let mpg = 10;
    exec(dist, fuel, mpg);
}
