mod common;

use common::Payload;
use rand::prelude::*;
use std::io::{Write};

use std::net::UdpSocket;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:61239")?;

    let mut sender_id = String::new();
    print!("Connection ID? ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut sender_id)?;

    let sender_id = sender_id.lines().next().unwrap_or("").to_string();

    if sender_id.is_empty() {
        println!("Connection ID must not be empty");
        return Ok(());
    }

    let bincode_config = common::bincode_config();
    let mut message_id = 0;

    let mut rng = rand::thread_rng();
    let series_id: i32 = rng.gen();

    loop {
        if message_id % 10 == 0 {
            print!(".");
            std::io::stdout().flush()?;
        }
        let payload: Payload = Payload {
            series_id,
            sender_id: sender_id.clone(),
            message_id,
        };
        let payload = bincode_config.serialize(&payload)?;
        socket.send_to(&payload, "chocolytech.info:61250")?;

        std::thread::sleep(std::time::Duration::from_millis(500));
        message_id += 1;
    }
}
