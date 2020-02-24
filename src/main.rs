use std::env;
use std::mem;
use std::net;

use c2rust_bitfields::BitfieldStruct;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, BitfieldStruct, Default)]
pub struct Header {
    pub size: u16,
    #[bitfield(name = "protocol", ty = "libc::c_ushort", bits = "0..=11")]
    #[bitfield(name = "addressable", ty = "libc::c_uchar", bits = "12..=12")]
    #[bitfield(name = "tagged", ty = "libc::c_uchar", bits = "13..=13")]
    #[bitfield(name = "origin", ty = "libc::c_uchar", bits = "14..=15")]
    pub protocol_addressable_tagged_origin: [u8; 2usize],
    pub source: u32,
    pub target: [u8; 8usize],
    pub reserved: [u8; 6usize],
    #[bitfield(name = "res_required", ty = "libc::c_uchar", bits = "0..=0")]
    #[bitfield(name = "ack_required", ty = "libc::c_uchar", bits = "1..=1")]
    #[bitfield(name = "_padding_one", ty = "libc::c_uchar", bits = "2..=5")]
    pub res_required_ack_required_padding: [u8; 1usize],
    pub sequence: u8,
    pub _padding_two: [u8; 8usize],
    pub message_type: u16,
    pub _padding_three: [u8; 2usize],
}

impl Header {
    fn new(message_type: u16, size: u16) -> Header {
        let mut h: Header = Header {
            message_type,
            size,
            ..Default::default()
        };

        h.set_protocol(1024);
        h.set_tagged(1);
        h.set_addressable(1);

        return h;
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SetPowerPayload {
    pub level: u16,
    pub duration: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SetPowerMessage {
    pub header: Header,
    pub payload: SetPowerPayload,
}

impl SetPowerMessage {
    fn new(level: u16) -> SetPowerMessage {
        let h = Header::new(117, mem::size_of::<SetPowerMessage>() as u16);

        let pw = SetPowerPayload {
            duration: 256,
            level,
        };

        let pwm = SetPowerMessage {
            header: h,
            payload: pw,
        };

        return pwm;
    }

    fn new_off_message() -> SetPowerMessage {
        Self::new(0x0000)
    }

    fn new_on_message() -> SetPowerMessage {
        Self::new(0xFFFF)
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                ::std::mem::size_of::<Self>(),
            )
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) => match s.as_ref() {
            "on" => send(SetPowerMessage::new_on_message().as_bytes()),
            "off" => send(SetPowerMessage::new_off_message().as_bytes()),
            s => println!("Unrecognized command {}", s),
        },
        None => println!("Need command"),
    }
}

fn send(msg: &[u8]) {
    let socket = net::UdpSocket::bind("0.0.0.0:0").expect("failed to bind host socket");

    match socket.send_to(msg, "192.168.1.255:56700") {
        Err(fail) => println!("failed sending {:?}", fail),
        _ => {}
    }
}
