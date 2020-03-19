mod common;

use postgres::{Client, NoTls};
use common::Payload;
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:61240")?;
    let mut buf = [0; 2048];


    let mut client = Client::connect("postgres://cobrand@192.168.1.15/cobrand", NoTls)?;

    let bincode_config = common::bincode_config();
    println!("listening port 61240...");

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];

        if let Ok(payload) = bincode_config.deserialize::<Payload>(buf) {
            println!("{:?} {:?}", payload, src);

            // client.execute(
            //     "INSERT"
        }
    }
}