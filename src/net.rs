use serde::ser::Serialize;
use std::net;

pub fn send_message<T: Serialize>(msg: T) {
    let bytes: Vec<u8> = bincode::serialize(&msg).expect("Failed to convert message to bytes");
    send(&bytes)
}

fn send(msg: &[u8]) {
    let socket = net::UdpSocket::bind("0.0.0.0:0").expect("Failed to bind host socket");
    socket
        .set_broadcast(true)
        .expect("Couldn't enable broadcast");

    match socket.send_to(msg, "192.168.1.255:56700") {
        Err(fail) => println!("Failed sending {:?}", fail),
        _ => {}
    }
}
