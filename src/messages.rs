use c2rust_bitfields::BitfieldStruct;
use serde::{Deserialize, Serialize};
use std::mem;

#[derive(Debug, Copy, Clone, BitfieldStruct, Default, Serialize, Deserialize)]
struct Header {
    size: u16,
    #[bitfield(name = "protocol", ty = "libc::c_ushort", bits = "0..=11")]
    #[bitfield(name = "addressable", ty = "libc::c_uchar", bits = "12..=12")]
    #[bitfield(name = "tagged", ty = "libc::c_uchar", bits = "13..=13")]
    #[bitfield(name = "origin", ty = "libc::c_uchar", bits = "14..=15")]
    protocol_addressable_tagged_origin: [u8; 2usize],
    source: u32,
    target: [u8; 8usize],
    reserved: [u8; 6usize],
    #[bitfield(name = "res_required", ty = "libc::c_uchar", bits = "0..=0")]
    #[bitfield(name = "ack_required", ty = "libc::c_uchar", bits = "1..=1")]
    #[bitfield(name = "_padding_one", ty = "libc::c_uchar", bits = "2..=5")]
    res_required_ack_required_padding: [u8; 1usize],
    sequence: u8,
    _padding_two: [u8; 8usize],
    message_type: u16,
    _padding_three: [u8; 2usize],
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct SetPowerPayload {
    level: u16,
    duration: u32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SetPowerMessage {
    header: Header,
    payload: SetPowerPayload,
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

    pub fn new_off_message() -> SetPowerMessage {
        Self::new(0x0000)
    }

    pub fn new_on_message() -> SetPowerMessage {
        Self::new(0xFFFF)
    }
}
