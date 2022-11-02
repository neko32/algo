pub mod document_build {

    use std::collections::BTreeMap;

    pub fn exec(dict: String) -> Vec<char> {
        let mut h: BTreeMap<char, i32> = BTreeMap::new();
        let dict_b = dict.as_bytes();
        for b in dict_b {
            let ch = *b as char;
            match h.get(&ch) {
                Some(n) => h.insert(ch, n + 1),
                None => h.insert(ch, 1),
            };
        }
        let keys = h.keys();
        let keyv: Vec<char> = keys.map(|x| *x).collect();
        println!("built char map is {:?}", h);
        keyv
    }

    pub fn run() {
        let s = "Todd told Tom to trot to the timber";
        println!("{:?}", exec(s.to_string()));
    }
}
