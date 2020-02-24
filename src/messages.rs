use c2rust_bitfields::BitfieldStruct;
use serde::{Deserialize, Serialize};
use std::mem;

use crate::scenes::Scene;

#[repr(C)]
#[derive(Debug, Copy, Clone, BitfieldStruct, Default, Serialize, Deserialize)]
struct Header {
    size: u16,
    #[bitfield(name = "protocol", ty = "libc::c_ushort", bits = "0..=11")]
    #[bitfield(name = "addressable", ty = "bool", bits = "12..=12")]
    #[bitfield(name = "tagged", ty = "bool", bits = "13..=13")]
    #[bitfield(name = "origin", ty = "libc::c_uchar", bits = "14..=15")]
    protocol_addressable_tagged_origin: [u8; 2usize],
    source: u32,
    target: [u8; 8usize],
    reserved: [u8; 6usize],
    #[bitfield(name = "res_required", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "ack_required", ty = "bool", bits = "1..=1")]
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
            source: 44444444,
            ..Default::default()
        };

        h.set_protocol(1024);
        h.set_tagged(true);
        h.set_addressable(true);
        h.set_res_required(true);

        return h;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct SetPowerPayload {
    level: u16,
    duration: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SetPowerMessage {
    header: Header,
    payload: SetPowerPayload,
}

impl SetPowerMessage {
    fn message_type() -> u16 {
        117
    }

    fn new(level: u16) -> SetPowerMessage {
        let header = Header::new(Self::message_type(), mem::size_of::<Self>() as u16);

        let payload = SetPowerPayload {
            duration: 256,
            level,
        };

        let msg = SetPowerMessage { header, payload };

        return msg;
    }

    pub fn new_off_message() -> SetPowerMessage {
        Self::new(0x0000)
    }

    pub fn new_on_message() -> SetPowerMessage {
        Self::new(0xFFFF)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
struct HSBK {
    hue: u16,
    saturation: u16,
    brightness: u16,
    kelvin: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
struct SetColorPayload {
    _padding: u8,
    color: HSBK,
    duration: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SetColorMessage {
    header: Header,
    payload: SetColorPayload,
}

impl SetColorMessage {
    fn message_type() -> u16 {
        102
    }

    pub fn new(brightness: u16, kelvin: u16) -> Self {
        let header = Header::new(Self::message_type(), mem::size_of::<Self>() as u16);

        let color = HSBK {
            brightness,
            kelvin,
            ..Default::default()
        };

        let payload = SetColorPayload {
            duration: 256,
            color,
            ..Default::default()
        };

        let msg = Self { header, payload };

        return msg;
    }

    pub fn new_scene(s: Scene) -> Self {
        Self::new(s.brightness, s.kelvin)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct StatePower {
    level: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StatePowerMessage {
    header: Header,
    payload: StatePower,
}
