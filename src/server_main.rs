mod common;

use postgres::{Client, NoTls};
use common::Payload;
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:61250")?;
    let mut buf = [0; 2048];


    let mut client = Client::connect("postgres://cobrand@192.168.1.15/cobrand", NoTls)?;

    let bincode_config = common::bincode_config();
    println!("listening port 61250...");

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];

        if let Ok(payload) = bincode_config.deserialize::<Payload>(buf) {
            let now = std::time::SystemTime::now();

            let r = client.execute(
                "INSERT INTO pote.poteudp (sender_id, timestamp, message_id, series_id, ip_address)
                VALUES ($1, $2, $3, $4, $5)",
                &[&payload.sender_id, &now, &payload.message_id, &payload.series_id, &src.ip()]
            );
            if let Err(e) = r {
                eprintln!("error while sending payload to database: {}", e);
            }
        }
    }
}