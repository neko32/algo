

use trie_rs::TrieBuilder;
pub fn exec(s:&[&str], query:&str) -> Vec<String> {
    let mut tb = TrieBuilder::new();
    for ss in s {
        tb.push(ss);
    }
    let trie = tb.build();
    let rez:Vec<Vec<u8>> = trie.common_prefix_search(query);
    let rezs:Vec<String> = rez.into_iter().map(|s|String::from_utf8(s).unwrap()).collect();
    return rezs
}


pub fn run() {
    exec(&["neko", "nekoya", "takoya", "tako", "pokoya", "nyanko", "nek"], "neko");
}
