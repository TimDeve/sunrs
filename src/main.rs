mod messages;
mod scenes;

use serde::ser::Serialize;
use std::env;
use std::io::ErrorKind;
use std::mem;
use std::net;
use std::thread;

use messages::{SetColorMessage, SetPowerMessage, StatePowerMessage};
use scenes::{BRIGHT, CHILL, COMPUTER, DARK, DAYLIGHT, READING};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) => match s.as_ref() {
            "on" => send_message(SetPowerMessage::new_on_message()),
            "off" => send_message(SetPowerMessage::new_off_message()),
            "day" => {
                send_message(SetColorMessage::new_scene(DAYLIGHT));
                send_message(SetPowerMessage::new_on_message());
            }
            "comp" => {
                send_message(SetColorMessage::new_scene(COMPUTER));
                send_message(SetPowerMessage::new_on_message());
            }
            "bright" => {
                send_message(SetColorMessage::new_scene(BRIGHT));
                send_message(SetPowerMessage::new_on_message());
            }
            "read" => {
                send_message(SetColorMessage::new_scene(READING));
                send_message(SetPowerMessage::new_on_message());
            }
            "dark" => {
                send_message(SetColorMessage::new_scene(DARK));
                send_message(SetPowerMessage::new_on_message());
            }
            "chill" => {
                send_message(SetColorMessage::new_scene(CHILL));
                send_message(SetPowerMessage::new_on_message());
            }
            s => print_error_and_exit(&format!("Unrecognized command '{}'", s), 64),
        },
        None => print_error_and_exit("Need command", 64),
    }
}

fn print_error_and_exit(s: &str, exit_code: i32) {
    eprintln!("{}", s);
    std::process::exit(exit_code);
}

fn listen(sock: &net::UdpSocket, mut buf: &mut [u8]) -> Option<usize> {
    // let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("no data received");
    let result = sock.recv(&mut buf);
    match result {
        // If `recv` was successfull, print the number of bytes received.
        // The received data is stored in `buf`.
        Ok(num_bytes) => {
            println!("I received {} bytes!", num_bytes);
            Some(num_bytes)
        }
        // If we get an error other than "would block", print the error.
        Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
            println!("Something went wrong: {}", err);
            None
        }
        // Do nothing otherwise.
        _ => None,
    }
}

fn send_message<T: Serialize>(msg: T) {
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

    let mut buf: Vec<u8> = vec![0; 1024];

    let socket_two = net::UdpSocket::bind("0.0.0.0:56701").expect("Failed to bind host socket");

    match listen(&socket_two, &mut buf) {
        Some(number_of_bytes) => {
            println!("{:x?}", &buf);

            let decoded = bincode::deserialize::<StatePowerMessage>(&buf);
            dbg!(decoded);
        }
        _ => {}
    }
}
