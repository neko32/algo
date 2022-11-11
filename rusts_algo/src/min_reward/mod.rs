
pub mod min_reward {

    use multiarray::Array1D;
    use std::cmp::max;

    pub fn exec(scores:&[i32]) -> i32 {
        let len = scores.len();
        let mut rewards = Array1D::new(len, 1);
        for i in 1..len {
            if scores[i] > scores[i - 1] {
                rewards[i] = rewards[i - 1] + 1;
            }
        }
        for i in (0..len - 1).rev() {
            if scores[i] > scores[i + 1] {
                rewards[i] = max(rewards[i], rewards[i + 1] + 1);
            }
        }

        let mut sumv = 0;
        for i in 0..len {
            print!("{} ", rewards[i]);
            sumv += rewards[i];
        }
        println!("");
        println!("sum is {}", sumv);
        sumv
    }

    pub fn run() {
        let scores = [8, 4, 2, 1, 3, 6, 7, 9, 5];
        let sumv = exec(&scores);
        println!("{:?} -> {}", scores, sumv);
    }

}