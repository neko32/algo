
use rand::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use std::collections::VecDeque;
use std::thread::{self, ScopedJoinHandle};
use std::sync::{Mutex, Arc};
use std::time::Duration;

pub fn exec(target:&str) {

    let q:VecDeque<String> = VecDeque::new();
    let shared_q = Arc::new(Mutex::new(q));

    thread::scope(|s|{
        let mut ths:Vec<ScopedJoinHandle<()>> = Vec::new();
        for _ in 0..3 {
            let shared_q = shared_q.clone();
            let th = s.spawn(move ||{
                loop {
                    let id = thread::current().id();
                    let mut rng = thread_rng();
                    let prefix = Alphanumeric.sample_string(&mut rng, 6);
                    let url = format!("{}.{}", prefix, target);
                    {
                        let mut p = shared_q.lock().unwrap();
                        println!("querying {url} by attacker {:?}", id);
                        p.push_back(url);
                    }
                    thread::sleep(Duration::from_millis(500));
                }
            });
            ths.push(th);
        }

        let cons = s.spawn(move ||{
            loop {
                {
                    let mut p = shared_q.lock().unwrap();
                    if let Some(data) = p.pop_front() {
                        println!("{data}");
                    }
                    println!("qsize - {}", p.len());
                    if p.len() > 1 {
                        panic!("overflow - your name server got hung");
                    }
                }
                thread::sleep(Duration::from_millis(800));
            }
        });
        ths.push(cons);

        for th in ths {
            th.join().unwrap();
        }
    });
}

pub fn run() {
    exec("tanuki.com");
}
