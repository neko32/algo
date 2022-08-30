
pub mod request_per_sec {

    pub fn exec(avg_msec:u32) -> u32 {
        let num_cpu:u32 = num_cpus::get() as u32;
        let rez = (1000 / avg_msec) * num_cpu;
        println!("num of cpus(cpu * core) = {}, rps = {}", num_cpu, rez);
        rez
    }

    pub fn run() {
        let n = 17;
        exec(n);
    }
}