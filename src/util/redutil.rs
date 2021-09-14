static mut CONNECTIONS: Vec<redis::aio::MultiplexedConnection> = vec![];

pub async fn init(){
    println!("Initializing Redis AIO Connections...");
    unsafe {
        let lowest_bin_client: redis::Client = redis::Client::open("redis://fragrunner.me:6001").unwrap();
        let lowest_bin_connection: redis::aio::MultiplexedConnection = lowest_bin_client.get_multiplexed_tokio_connection().await.unwrap();
       
        let auctions_client: redis::Client = redis::Client::open("redis://fragrunner.me:6002").unwrap();
        let auctions_connection: redis::aio::MultiplexedConnection = auctions_client.get_multiplexed_tokio_connection().await.unwrap();
       
        let ended_auctions_client: redis::Client = redis::Client::open("redis://fragrunner.me:6003").unwrap();
        let ended_auctions_connection: redis::aio::MultiplexedConnection = ended_auctions_client.get_multiplexed_tokio_connection().await.unwrap();


        CONNECTIONS.push(lowest_bin_connection);
        CONNECTIONS.push(auctions_connection);
        CONNECTIONS.push(ended_auctions_connection);

        
    println!("Successfully initialized Redis AIO Connections!");
    }
}


pub fn get_connection(t: Connection) -> redis::aio::MultiplexedConnection {
    let slot: usize = get_slot_from_connection(t);
    if slot == 99 {
        println!("Invalid Connection Entry found!  Slot: {}", slot);
    }
    unsafe{
        return CONNECTIONS[slot].clone();
    }
}

pub fn get_slot_from_connection(t: Connection) -> usize {
    if let Connection::LowestBin = t {return 0;}
    if let Connection::Auctions = t {return 1;}
    if let Connection::EndedAuctions = t {return 2;}

    return 99;
}


pub enum Connection{
    LowestBin,
    Auctions,
    EndedAuctions
}