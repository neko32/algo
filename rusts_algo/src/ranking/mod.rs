

pub fn exec(scores:&Vec<i32>) -> Vec<(i32, u32)> {
    let mut ranks:Vec<u32> = Vec::from_iter(std::iter::repeat(0).take(102));
    ranks[101] = 1;
    for score in scores {
        ranks[*score as usize] += 1;
    }
    for i in (0..=100).rev() {
        ranks[i] += ranks[i + 1];
    }
    let mut rez:Vec<(i32, u32)> = Vec::new();
    for score in scores {
        rez.push((*score, ranks[(*score + 1) as usize]));
    }
    println!("{:?}", rez);
    rez
}

pub fn run() {
    let scores = vec![56, 25, 67, 88, 100, 61, 55, 67, 76, 56];
    exec(&scores);
}