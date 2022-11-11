
pub fn exec(a:&mut [i32], order:&[i32]) {
    let a_len = a.len();
    let order_len = order.len();
    let mut cnt:Vec<i32> = Vec::from_iter(std::iter::repeat(0).take(order_len));
    for i in 0..a_len {
        'lp:for j in 0..order_len {
            if a[i] == order[j] {
                cnt[j] += 1;
                break 'lp;
            }
        }
    }
    println!("count array was build - {:?}", cnt);

    for i in 0..order_len {
        let v = order[i];
        let offset_idx:i32 = cnt[0..i].iter().sum();
        let g = cnt[i];
        for j in offset_idx..offset_idx + g {
            a[j as usize] = v;
        }
    }
}

pub fn run() {
    let mut a = [1, 0, 0, -1, -1, 0, 1, 1];
    let order = [0, 1, -1];
    println!("order - {:?}", order);
    println!("before - {:?}", a);
    exec(&mut a, &order);
    println!("after - {:?}", a);
}