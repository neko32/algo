
pub mod min_breakdown_sum {

    pub fn exec(minutes:u32) -> u32 {
        let hour = minutes / 60;
        let mins = minutes % 60;
        let hour_s = hour.to_string();
        let mins_s = mins.to_string();
        let h_sum = hour_s.chars().map(|c|c.to_digit(10).unwrap()).sum::<u32>();
        let m_sum = mins_s.chars().map(|c|c.to_digit(10).unwrap()).sum::<u32>();
        let rez = h_sum + m_sum;
        println!("input - {}, hours - {}, mins - {}, rez - {}", minutes, hour, mins, rez);
        rez
    }

    pub fn run() {
        let n:u32 = 240;
        println!("{}", exec(n));
        let m:u32 = 808;
        println!("{}", exec(m));
    }
}