use nbt::from_gzip_reader;
use lazy_static::lazy_static;

use std::result::Result as StdResult;
use std::io::Cursor;

use std::thread;
use std::time::Duration;

use crate::util::auction::*;
use crate::objects::item::*;
use crate::objects::data::*;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
    .gzip(true)
    .brotli(true)
    .build()
    .unwrap();
 }


 #[tokio::main]
pub async fn startThread(page: i32){
   println!("Starting...");
       thread::spawn(|| async  {
        loop {
            let result: AuctionResult = get(1).await;

           thread::sleep(Duration::from_secs(1));
        }
    });
}

pub async fn get(page: i32) -> AuctionResult {
    let res = HTTP_CLIENT.get(format!("https://api.hypixel.net/skyblock/auctions?page={}", page)).send().await.unwrap();
    let text = res.text().await.unwrap();
    return serde_json::from_str(&text).unwrap();
 }
 
 pub async fn parse(item: AuctionItem) -> ConfiguredAuctionItem {
    let byte: StdResult<Vec<u8>, _> = item.bytes.clone().into();
    let _nbt: PartialNbt  = from_gzip_reader(Cursor::new(byte.unwrap())).unwrap();
    
    let i: ConfiguredAuctionItem = ConfiguredAuctionItem {
       nbt: _nbt,
       item: item
    };

    return i;
 }