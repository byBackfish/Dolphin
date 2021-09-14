extern crate redis;
use dolphin::util::threader::{start};
use dolphin::util::websocket::{start as start_websocket};
use dolphin::util::redutil::{init};

#[tokio::main]
async fn main() {
   let _init = init().await;
  start_websocket();

  for i in 0..3 {
   start(i);
   std::thread::sleep(std::time::Duration::new(5, 0));
  }

  // start_ended();
   loop {
     // Some random dangerous shitty code here  

   }
}