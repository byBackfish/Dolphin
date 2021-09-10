use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};
use eventual::Timer;
use std::thread;

use nbt::Blob;

    static mut USERS: Vec<User> = vec![];
     
struct User {
    sender: Sender,
    username: String,
    key: String,
    uuid: String
}
struct Server {
    out: Sender,
}
impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Message: {}", msg);
        let string: String = msg.to_string();
      
        if string.starts_with("authme") {
            let split: Vec<&str> = string.split(":").collect();
            let username: String = split[1].to_string();
            let key: String = split[2].to_string();
            let uuid: String = split[3].to_string();
    
            let user = User {
                sender: self.out.clone(),
                username,
                key,
                uuid
            };
            println!("Username: {username} | Key: {key} | UUID: {uuid}", username=user.username, key=user.key, uuid=user.uuid);
            unsafe {
                USERS.push(user);
                let _r = self.out.send("auth:success");
            }
        }
        self.out.send(msg)
    }

}


pub fn start() {
    println!("Connecting...");
    listen("127.0.0.1:2794", |out| { Server { out: out } }).unwrap();
    println!("Yep");
}

pub fn send(message: &str){
    unsafe {
        for x in USERS.iter() {
            let user: &User = x;
         //   let _a = user.sender.send(message);
        }
    }
}