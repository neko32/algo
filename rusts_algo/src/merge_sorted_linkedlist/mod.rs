
pub fn exec(l1:&mut Vec<i32>, l2:&mut Vec<i32>) -> Vec<i32> {
    let l1_len = l1.len();
    let l2_len = l2.len();

    let (src, dest) = if l1_len < l2_len { (l1, l2) } else { (l2, l1) };

    let mut src_idx = 0;
    let mut dest_idx = 0;
    let src_len = src.len();
    let dest_len = dest.len();

    while src_idx < src_len {
        while dest_idx + 1 < dest_len && src[src_idx] > dest[dest_idx + 1] {
            println!("dest_idx + 1 {}", dest_idx);
            dest_idx += 1;
        }
        println!("insert {}", src[src_idx]);
        dest.insert(dest_idx + 1, src[src_idx]);
        println!("src + 1 from {} and dest + 1 {}", src_idx, dest_idx);
        dest_idx += 1;
        src_idx += 1;
    }
    println!("{:?}", dest);
    dest.to_vec()
}

pub fn run() {
    let mut l1 = vec![2, 6, 7, 8];
    let mut l2 = vec![1, 3, 4, 5, 9, 10];
    exec(&mut l1, &mut l2);
}