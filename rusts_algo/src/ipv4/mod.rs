
use regex::Regex;
pub fn exec(ip:&str) -> bool {
    const REGEX_STR:&str = r"^(25[0-5]|2[0-4][0-9]|[2-9][0-9]?|1[0-9]?[0-9]?|0)(\.(25[0-5]|2[0-4][0-9]|[2-9][0-9]?|1[0-9]?[0-9]?|0)){3}$";
    let regex:Regex = Regex::new(REGEX_STR).unwrap();
    let rez = regex.find(ip).is_some();
    println!("{ip} is valid ipv4? - {rez}");
    rez
}

pub fn run() {
    let validones = vec!["192.168.0.1", "0.0.0.0", "255.255.255.255",
    "129.20.38.0", "240.240.240.240", "1.2.3.4", "10.20.30.40", "99.99.99.99",
    "127.0.0.1"];
    println!("must be all valid");
    validones.iter().for_each(|ip|{
        exec(ip);
    });
    println!("msut be all invalid");
    let invalidones = vec!["-1.2.3.4", "256.2.1.8", "79.79.79.790", "192.168.8", "192.168.256.8", "260.2.2.100",
    "50", "0.0.0.256", "1000.100000.1000000000.10", "a.b.c.d", "192.01.25.33", "08.10.2.25", "0001.0004.0008.0002"];
    invalidones.iter().for_each(|ip|{
        exec(ip);
    });
}
