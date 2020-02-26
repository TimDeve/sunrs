use c2rust_bitfields::BitfieldStruct;
use serde::{Deserialize, Serialize};

use std::ffi::CStr;
use std::mem;

use crate::scenes::Scene;

fn extract_string_from_bytes(bytes: &[u8]) -> Option<String> {
    for (i, val) in bytes.iter().enumerate() {
        if *val == b'\0' {
            let str = &bytes[0..i + 1];

            return match CStr::from_bytes_with_nul(&str) {
                Ok(s) => Some(s.to_string_lossy().to_string()),
                Err(_) => None,
            };
        }
    }

    None
}

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
            source: 4444,
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
struct StatePowerPayload {
    level: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StatePowerMessage {
    header: Header,
    payload: StatePowerPayload,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GetServiceMessage {
    header: Header,
}

impl GetServiceMessage {
    fn message_type() -> u16 {
        2
    }

    pub fn new() -> Self {
        let header = Header::new(Self::message_type(), mem::size_of::<Self>() as u16);
        let msg = Self { header };
        return msg;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StateServicePayload {
    service: u8,
    port: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StateServiceMessage {
    header: Header,
    payload: StateServicePayload,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GetLocationMessage {
    header: Header,
}

impl GetLocationMessage {
    fn message_type() -> u16 {
        48
    }

    pub fn new() -> Self {
        let header = Header::new(Self::message_type(), mem::size_of::<Self>() as u16);
        let msg = Self { header };
        return msg;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StateLocationPayload {
    location: [u8; 16usize],
    label: [u8; 32usize],
    updated_at: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StateLocationMessage {
    header: Header,
    payload: StateLocationPayload,
}

impl StateLocationMessage {
    pub fn get_label(&self) -> Option<String> {
        extract_string_from_bytes(&self.payload.label)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GetLabelMessage {
    header: Header,
}

impl GetLabelMessage {
    fn message_type() -> u16 {
        48
    }

    pub fn new() -> Self {
        let header = Header::new(Self::message_type(), mem::size_of::<Self>() as u16);
        let msg = Self { header };
        return msg;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StateLabelPayload {
    label: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StateLabelMessage {
    header: Header,
    payload: StateLocationPayload,
}

impl StateLabelMessage {
    pub fn get_label(&self) -> Option<String> {
        extract_string_from_bytes(&self.payload.label)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GetMessage {
    header: Header,
}

impl GetMessage {
    fn message_type() -> u16 {
        101
    }

    pub fn new() -> Self {
        let header = Header::new(Self::message_type(), mem::size_of::<Self>() as u16);
        let msg = Self { header };
        return msg;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StatePayload {
    color: HSBK,
    _reserved_one: i16,
    power: u16,
    label: [u8; 32usize],
    _reserved_two: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StateMessage {
    header: Header,
    payload: StatePayload,
}

impl StateMessage {
    pub fn get_label(&self) -> Option<String> {
        extract_string_from_bytes(&self.payload.label)
    }
}
