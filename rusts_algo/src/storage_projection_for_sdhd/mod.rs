
const SD_GB_PER_HOUR:u32 = 10;
const HD_GB_PER_HOUR:u32 = 20;
const SD_MB_PER_SEC:u32 = 2;
const HD_MB_PER_SEC:u32 = 5;

pub fn exec(num_sd:u32, num_hd:u32) -> (u32, u32) {
    let sd_per_h = num_sd * SD_GB_PER_HOUR;
    let sd_per_s = num_sd * SD_MB_PER_SEC;
    let hd_per_h = num_hd * HD_GB_PER_HOUR;
    let hd_per_s = num_hd * HD_MB_PER_SEC;
    let per_h_total = sd_per_h + hd_per_h;
    let per_s_total = sd_per_s + hd_per_s;
    println!("per hour total - {}(HD/h:{}, SD/h:{},num of HD:{}, num of SD:{})", per_h_total, hd_per_h, sd_per_h, num_hd, num_sd);
    println!("per sec total - {}(HD/s:{}, SD/s:{},num of HD:{}, num of SD:{})", per_s_total, hd_per_s, sd_per_s, num_hd, num_sd);
    (per_h_total, per_s_total)
}

pub fn run() {
    exec(27, 42);
}
