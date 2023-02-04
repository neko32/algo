
pub fn exec(a:&[i32], b:&[i32]) -> bool {
    let mut a_sorted:Vec<i32> = a.iter().cloned().collect();
    let mut b_sorted:Vec<i32> = b.iter().cloned().collect();
    a_sorted.sort();
    b_sorted.sort();
    a_sorted == b_sorted || a.iter().zip(b.iter()).filter(|(x,y)|x != y).count() < 2
}

pub fn run() {
    println!("{}", exec(&[1, 2, 3], &[2, 1, 3]));
    println!("{}", exec(&[1, 2, 3], &[1, 2, 3]));
    println!("{}", exec(&[1, 2, 2], &[2, 1, 1]));
}
