
use std::sync::{Arc, Mutex, PoisonError};
use std::collections::HashMap;

pub fn exec() -> Result<(), anyhow::Error> {
    let mut d:CompareSwapData = CompareSwapData::new();
    d.initialize("owner", "0001", "tanuki").unwrap();
    println!("{:?}", d.get("owner", "0001").unwrap().unwrap());
    let rez = d.update("owner", "0001", "ushi", "0002", "ushi");
    assert!(rez.is_err());
    let rez2 = d.update("owner", "0002", "tanuki", "0002", "ushi");
    assert!(rez2.is_err());
    println!("{:?}", d.get("owner", "0001").unwrap().unwrap());
    d.update("owner", "0001", "tanuki", "0002", "ushi").unwrap();
    println!("{:?}", d.get("owner", "0002").unwrap().unwrap());

    Ok(())
}

pub fn run() {
    exec().unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub name: String,
    pub rs_ver: String
}

impl Key {
    pub fn new(n:&str, rs_ver:&str) -> Self {
        Key { name: n.to_string(), rs_ver:rs_ver.to_string() }
    }
}

#[derive(Debug)]
pub struct CompareSwapData {
    pub data:Arc<Mutex<HashMap<Key, String>>>,
}

impl CompareSwapData {
    pub fn new() -> Self {
        CompareSwapData { data: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn initialize(&mut self, init_key:&str, init_rs_ver:&str, init_data:&str) -> Result<(), anyhow::Error> {
        let mut l = self.data.lock().unwrap();
        let init_key = Key::new(init_key, init_rs_ver);
        l.insert(init_key, init_data.to_string());
        Ok(())
    }

    pub fn get(&self, key:&str, current_rs_ver:&str) -> Result<Option<String>, anyhow::Error> {
        let l = self.data.lock();
        match l {
            Ok(g) => {
                let r = Key::new(key, current_rs_ver);
                let v = g.get(&r);
                Ok(v.map(|x|x.clone()))
            },
            Err(_) => {
                let anyhow_e = anyhow::Error::new(PoisonError::new("err"));
                Err(anyhow_e)
            }
        }
    }

    pub fn update(&mut self, key:&str, current_rs_ver:&str, current_name:&str, next_rs_ver:&str, next_name:&str) -> Result<(), anyhow::Error> {
        let l = self.data.lock();
        let k = Key::new(key, current_rs_ver);
        match l {
            Ok(mut g) => {
                match g.get(&k) {
                    Some(found) if found == current_name => {
                        let newk = Key::new(key, next_rs_ver);
                        g.insert(newk, next_name.to_string());
                        g.remove(&k);
                        Ok(())
                    },
                    _ => {
                        let anyhow_e = anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::NotFound, "key not found"));
                        Err(anyhow_e)
                    }
                }
            },
            Err(_) => {
                let anyhow_e = anyhow::Error::new(PoisonError::new("err"));
                Err(anyhow_e)
            }

        } 
    }

}