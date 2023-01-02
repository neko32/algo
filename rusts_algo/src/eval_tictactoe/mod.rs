
use regex::Regex;
pub fn exec(s:&str) -> bool {
    let regex:Regex = Regex::new(r"^.*O{3}.*$|O..O..O..|.O..O..O.|..O..O..O|O...O...O|..O.O.O..").unwrap();
    regex.find(s).is_some()
}

pub fn run() {
    let s:Vec<&str> = vec![
        "OXXXOXXXO", "XXXXXXOOO", "XXXOOOXXX",
        "OOOXXXXXX", "XOXXOXXOX", "XXOXOXOXX",
        "OXXOXXOXX", "XXOXXOXXO", "XXOXXOOXX"
    ];
    s.iter().for_each(|s|{
        let rez = exec(s);
        println!("{s} - {rez}");
    });
}
