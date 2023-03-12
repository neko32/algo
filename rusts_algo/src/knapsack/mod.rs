
use ndarray::Array2;

pub fn exec(w:Vec<i32>, v:Vec<i32>, target_w:i32) -> i32 {
    let items:Vec<Item> = w.into_iter().zip(v.into_iter()).map(|(w, v)|Item::new(w, v)).collect();
    let item_len = items.len();
    let mut dp:Array2<i32> = Array2::zeros([item_len + 1, target_w as usize + 1 as usize]);

    for (idx, item) in items.iter().enumerate() {
        for cur_target_w in 0..=target_w {
            if item.w <= cur_target_w {
                let x = dp[[idx, cur_target_w as usize - item.w as usize]] + item.v;
                let y = dp[[idx, cur_target_w as usize]];
                dp[[idx + 1, cur_target_w as usize]] = x.max(y);
            } else{
                dp[[idx + 1, cur_target_w as usize]] = dp[[idx, cur_target_w as usize]];
            }
        }
    }
    println!("{}", dp.view());
    dp[[item_len, target_w as usize]]
}

#[derive(Debug)]
struct Item {
    w:i32,
    v:i32,
}

impl Item {
    fn new(w:i32, v:i32) -> Self {
        Self { w: w, v: v}
    }
}

pub fn run() {
    exec(vec![2, 1, 3, 2, 1, 5], vec![3, 2, 6, 1, 3, 85], 9);
}

