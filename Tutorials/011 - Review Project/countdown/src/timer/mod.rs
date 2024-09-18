use std::{thread, time};

pub mod functions {
  pub fn sleep(seconds: u32){
    for _ in (0..seconds).rev() {
      super::thread::sleep(super::time::Duration::from_secs(1));
    }
  }
  
  pub fn countdown(seconds: u32){
    for i in (0..seconds).rev() {
      println!("{} seconds remaining", i + 1);
      sleep(1);
    }
    println!("\nTimer finished.");
  }
}