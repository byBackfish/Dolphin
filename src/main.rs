extern crate redis;
use redis::Commands;
use lazy_static::lazy_static;
use nbt::from_gzip_reader;
use std::result::Result as StdResult;
use std::time::Instant;
use std::io::Cursor;
use dolphin::util::auction::*;
use eventual::Timer;
use std::thread;

use dolphin::util::threader::{startThread};
/*
lazy_static! {
   static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
   .gzip(true)
   .brotli(true)
   .build()
   .unwrap();
}*/


fn main(){
   startThread(1);
/*let LOWEST_BIN_CLIENT: redis::Client = redis::Client::open("redis://fragrunner.me:6001").unwrap();
let mut LOWEST_BIN_CONNECTION: redis::Connection = LOWEST_BIN_CLIENT.get_connection().unwrap();

   thread::spawn(|| {
      start();
   });

thread::spawn(move || async { 
         let mut current = 0;
         let mut max = 0;
         println!("Timing...");
             let timer = Timer::new();
             let ticks = timer.interval_ms(1000).iter();
             for _ in ticks {
               let now = Instant::now();
               let result: AuctionResult = get(current).await;
            
               let auctions = result.auctions;
               println!("Request #{} took {}ms for {} Items", current, now.elapsed().as_millis(), auctions.len());
               for x in auctions.into_iter() {
                  let mut item: AuctionItem = x;
                  let byte: StdResult<Vec<u8>, _> = item.bytes.clone().into();
                  let _nbt: PartialNbt  = from_gzip_reader(Cursor::new(byte.unwrap())).unwrap();
                  
                  let i: ConfiguredAuctionItem = ConfiguredAuctionItem {
                     nbt: _nbt,
                     item: item
                  };

                  send(serde_json::to_string(&i));
            
               }
               max = result.totalPages;
               current = current + 1;
               if current > max {
                  current = 0;
               }
             }
         });*/
   
         thread::spawn(|| {
            loop {
             
            }
         }).join();
} 

/*
pub async fn get(page: i32) -> AuctionResult {
   let res = HTTP_CLIENT.get(format!("https://api.hypixel.net/skyblock/auctions?page={}", page)).send().await.unwrap();
   let text = res.text().await.unwrap();
   return serde_json::from_str(&text).unwrap();
}
*/