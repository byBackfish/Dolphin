use lazy_static::lazy_static;
use nbt::from_gzip_reader;

use std::io::Cursor;
use std::result::Result as StdResult;

use crate::objects::auction::*;
use crate::objects::data::*;
use crate::objects::item::*;

use futures::executor::block_on;
use std::collections::HashMap;
use std::hash::Hash;

use crate::redutil::{get_connection, Connection};
use redis::AsyncCommands;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .gzip(true)
        .brotli(true)
        .build()
        .unwrap();
}

pub fn start(page: i32) {
    println!("Starting {}", page);
    std::thread::spawn(move || block_on(do_shit(page)));
}

pub async fn get(page: i32) -> AuctionResult {
    let res = HTTP_CLIENT
        .get(format!(
            "https://api.hypixel.net/skyblock/auctions?page={}",
            page
        ))
        .send()
        .unwrap();
    let text = res.text().unwrap();
    return serde_json::from_str(&text).unwrap();
}

pub async fn get_ended() -> EndedAuctionResult {
    let res = HTTP_CLIENT
        .get(format!("https://api.hypixel.net/skyblock/auctions_ended"))
        .send()
        .unwrap();
    let text = res.text().unwrap();
    return serde_json::from_str(&text).unwrap();
}

pub fn parse(item: AuctionItem) -> ConfiguredAuctionItem {
    let byte: StdResult<Vec<u8>, _> = item.bytes.clone().into();
    let _nbt: PartialNbt = from_gzip_reader(Cursor::new(byte.unwrap())).unwrap();

    let mut i: ConfiguredAuctionItem = ConfiguredAuctionItem {
        nbt: _nbt,
        item: item,
    };

    let id: &String = &i.nbt.i[0].tag.extra_attributes.id;

    if id == "ENCHANTED_BOOK" {
        let enchants = i.nbt.i[0]
            .tag
            .extra_attributes
            .enchantments
            .as_ref()
            .unwrap();
        for (enchant, level) in enchants {
            i.nbt.i[0].tag.extra_attributes.id =
                format! {"{};{}", enchant.to_uppercase(), level}.to_string();
            break;
        }
    }

    return i;
}

pub fn parse_ended(item: EndedAuctionItem) -> ConfiguredEndedAuctionItem {
    let byte: StdResult<Vec<u8>, _> = item.bytes.clone().into();
    let _nbt: PartialNbt = from_gzip_reader(Cursor::new(byte.unwrap())).unwrap();

    let mut i: ConfiguredEndedAuctionItem = ConfiguredEndedAuctionItem {
        nbt: _nbt,
        item: item,
    };

    
    let id: &String = &i.nbt.i[0].tag.extra_attributes.id;

    if id == "ENCHANTED_BOOK" {
        let enchants = i.nbt.i[0]
            .tag
            .extra_attributes
            .enchantments
            .as_ref()
            .unwrap();
        for (enchant, level) in enchants {
            i.nbt.i[0].tag.extra_attributes.id =
                format! {"{};{}", enchant.to_uppercase(), level}.to_string();
            break;
        }
    }

    return i;
}

pub async fn do_shit(page: i32) {
    let mut connection = get_connection(Connection::LowestBin).clone();
    let mut auctions = get_connection(Connection::Auctions).clone();
    let result: AuctionResult = get(page).await;
    for x in result.auctions.into_iter() {
        let item: ConfiguredAuctionItem = parse(x);
        let id: &String = &item.nbt.i[0].tag.extra_attributes.id;

        let mut lowest_bin: i64 = connection.get(id).await.unwrap_or(-1);

        let stringified: String = serde_json::to_string(&item).unwrap();

        let _res: i32 = auctions.lpush(id, stringified).await.unwrap();


        let _a = get_price(&item).await;

        //   send(&format!{"Found Item {} with Starting Bid of {} on Page {} and a Price of {} which has a Lowest Bin of {}", id, item.item.price, page, item.item.price, lowest_bin});
        //   println!("Found Item {} with Starting Bid of {} on Page {} and a Price of {} which has a Lowest Bin of {}", id, item.item.price, page, item.item.price, lowest_bin);
        if lowest_bin > item.item.price || lowest_bin == -1 {
            lowest_bin = item.item.price;
            let _res: String = connection.set(id, lowest_bin).await.unwrap();
        }
    }
}

pub fn start_ended() {
    std::thread::spawn(move || block_on(save_ended_auctions()));
}

pub async fn save_ended_auctions() {
    let mut auctions = get_connection(Connection::EndedAuctions).clone();
    let result: EndedAuctionResult = get_ended().await;

    for x in result.auctions.into_iter() {
        let item: ConfiguredEndedAuctionItem = parse_ended(x);
        let id: &String = &item.nbt.i[0].tag.extra_attributes.id;

        let stringified: String = serde_json::to_string(&item).unwrap();
        let _res: i32 = auctions.lpush(id, stringified).await.unwrap();
        println!(
            "Found Ended Item {} with Starting Bid of {} and a Price of {}",
            id, item.item.price, item.item.price
        );
    }
}

pub async fn get_price(item: &ConfiguredAuctionItem) -> i64 {
    if item.nbt.i[0].tag.extra_attributes.id == "PET" {return -1;}
    let mut similar: Vec<ConfiguredEndedAuctionItem> = get_best_auctions(item).await;

    if similar.len() < 2 {
    //    println!("Item {} has not enough refrences ({}) and will therefore be skipped.", item.nbt.i[0].tag.display.name, similar.len());
        return -1;
    }

    similar.sort_by_key(|x| x.item.price);
    let mid = similar.len() / 2;

    let median: &ConfiguredEndedAuctionItem = &similar[mid];
    println!("Matched Item {} with the Item {} with the Price of {}. Had {} Refrences.", item.nbt.i[0].tag.display.name, median.nbt.i[0].tag.display.name, median.item.price, similar.len());

    return median.item.price;
}

fn keys_match<T: Eq + Hash, U, V>(map1: &HashMap<T, U>, map2: &HashMap<T, V>) -> bool {
    map1.len() == map2.len()
        && map1.keys().all(|k| map2.contains_key(k))
        && map2.keys().all(|k| map1.contains_key(k))
}

async fn get_best_auctions(item: &ConfiguredAuctionItem) -> Vec<ConfiguredEndedAuctionItem> {
    let mut connection = get_connection(Connection::EndedAuctions);
    let vec: Vec<String> = connection
        .lrange(&item.nbt.i[0].tag.extra_attributes.id, 0, -1)
        .await
        .unwrap();

    let extra: &PartialExtraAttr = &item.nbt.i[0].tag.extra_attributes;
    let mut relevant: Vec<ConfiguredEndedAuctionItem> = Vec::new();
    let mut ids: Vec<String> = Vec::new();

    for x in vec.iter() {
        let ended: ConfiguredEndedAuctionItem = serde_json::from_str(x).unwrap();
        if !ids.contains(&ended.item.uuid) { 
        ids.push(ended.item.uuid.clone().to_string());
        let ended_extra: &PartialExtraAttr = &item.nbt.i[0].tag.extra_attributes;

        if ended_extra.recombed.unwrap_or(0) == extra.recombed.unwrap_or(0)
            && ended_extra.hotpotato.unwrap_or(0) == extra.hotpotato.unwrap_or(0)
            && ended_extra.reforge.as_ref().unwrap_or(&"None".to_string())
                == extra.reforge.as_ref().unwrap_or(&"None".to_string())
            && ended_extra.stars.unwrap_or(0) == extra.stars.unwrap_or(0)
            && keys_match(
                extra.enchantments.as_ref().unwrap_or(&HashMap::new()),
                ended_extra.enchantments.as_ref().unwrap_or(&HashMap::new()),
            )
        {
            relevant.push(ended);
        }
    }
    }

   // relevant =  relevant.into_iter().filter(|x| relevant.contains(&x)).collect();

    return relevant;
}
