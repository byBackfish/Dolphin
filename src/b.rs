extern crate redis;
use redis::Commands;

fn main(){
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let _ : () = con.set("test", 42).unwrap();
    let res: i32 = con.get("test").unwrap();

    println!("{}", res);
}