use std::time::{SystemTime, Duration};
use num_traits::Num;
use std::fmt::Display;

// Cacheable trait
pub trait Cacheable {
    fn is_expired(&self, insertion_time: SystemTime, time_to_expire: Duration) -> bool;
}

impl<T: Num> Cacheable for T {
    fn is_expired(&self, insertion_time: SystemTime, time_to_expire: Duration) -> bool {
        let now = SystemTime::now();
        match now.duration_since(insertion_time) {
            Ok(duration) => duration > time_to_expire,
            Err(_) => panic!("Error to get expired status!")
        } 
    }
}

// Cache struct / item
pub struct Cache<T: Cacheable + Display> {
    pub value: T,
    pub insertion_time: SystemTime,
    pub time_to_expire: Duration
}

impl<T: Cacheable + Display> Cache<T> {
    pub fn new(value: T, time_to_expire: Duration) -> Self {
        return Cache {
            value,
            insertion_time: SystemTime::now(),
            time_to_expire
        };
    }

    pub fn is_valid(&self) -> bool {
        return !self.value.is_expired(self.insertion_time, self.time_to_expire);
    }
}

// Cache system struct
pub struct CacheSystem<T: Cacheable + Display> {
    pub items: Vec<Cache<T>>
}

impl<T: Cacheable + Display> CacheSystem<T> {
    pub fn new() -> Self {
        CacheSystem { items: Vec::new() }
    }

    pub fn insert_new(&mut self, value: T, seconds_to_expire: u64){
        let cache = Cache::new(value, Duration::from_secs(seconds_to_expire));
        self.items.push(cache);
    }

    pub fn print_items(&self){
        let mut count: u32 = 0;
        for item in &self.items {
            println!("{count} - {0}, is valid? {1}", item.value, item.is_valid());
            count += 1;
        }
    }
}

fn main() {
    let mut cache_system : CacheSystem<i64> = CacheSystem::new(); 

    loop {
        let mut res = String::new();
        std::io::stdin().read_line(&mut res).expect("Failed to read line!");
        let res : char = res.trim().parse().expect("Failed to convert input to char!");

        match res {
            'a' => { cache_system.insert_new(1, 5); },
            'h' => { println!("a: append item to cache\nh: help\nc: close application\np: print cache items"); },
            'p' => { cache_system.print_items(); },
            'c' => { println!("Closing..."); break; },
            _ => {}
        }
    };
}