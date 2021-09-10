use lazy_static::lazy_static;
use redis::Client;

lazy_static!{
 pub static ref LOWEST_BIN_CLIENT: redis::Client = redis::Client::open("redis://fragrunner.me:6001").unwrap();
 pub static ref LOWEST_BIN_CONNECTION: redis::Connection = LOWEST_BIN_CLIENT.get_connection().unwrap();

 pub static ref AUCTIONS_CLIENT: redis::Client = redis::Client::open("redis://fragrunner.me:6002").unwrap();
 pub static ref AUCTIONS_CONNECTION: redis::Connection = AUCTIONS_CLIENT.get_connection().unwrap();

 pub static ref ENDED_AUCTIONS_CLIENT: redis::Client = redis::Client::open("redis://fragrunner.me:6002").unwrap();
 pub static ref ENDED_AUCTIONS_CONNECTION: redis::Connection = ENDED_AUCTIONS_CLIENT.get_connection().unwrap();
}

