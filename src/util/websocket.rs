use ws::{listen, Handler, Sender, Result, Message};

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
    println!("Starting Websocket...");
    let _res = std::thread::spawn(|| {
    println!("Websocket started!");
        listen("127.0.0.1:2794", |out| { Server { out: out } }).unwrap()
    });
}

pub fn send(message: &str){
    unsafe {
        for x in USERS.iter() {
            let _a = x.sender.send(message);
        }
    }
}