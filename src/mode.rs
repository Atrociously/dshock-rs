#![allow(clippy::upper_case_acronyms)]

use crate::{Buttons, DPad, ControllerState, Triggers, Sticks, Vec3};

mod secret {
    pub trait Secret {}
}

pub trait Mode: secret::Secret {
    fn parse(state: &mut ControllerState, packet: [u8; 64]);
}

pub struct Usb;
pub struct Bt;

impl secret::Secret for Usb {}
impl Mode for Usb {
    fn parse(state: &mut ControllerState, packet: [u8; 64]) {
        todo!("{state:?}, {packet:?}")
    }
}

impl secret::Secret for Bt {}
impl Mode for Bt {
    // for implementation details see https://www.psdevwiki.com/ps4/DS4-BT#HID_Report_header_.26_footer
    fn parse(state: &mut ControllerState, packet: [u8; 64]) {
        println!("Gyro X: {:x} {:x} Y: {:x} {:x}", packet[17], packet[18], packet[19], packet[20]);
        state.stick_left.x = packet[5];
        state.stick_left.y = packet[6];

        state.stick_right.x = packet[7];
        state.stick_right.y = packet[8];

        state.buttons = Buttons(packet[9] >> 4);
        state.direction = DPad::from(packet[9]);
        state.triggers = Triggers::from(packet[10]);
        state.sticks = Sticks((packet[10] & 0xC0) >> 6); // select only sticks and shift
        state.buttons |= Buttons(packet[10] & 0x30); // select only option and share bits
        state.buttons |= Buttons((packet[11] & 0x01) << 5); // select only ps button and shift

        state.trigger_z_left = packet[12];
        state.trigger_z_right = packet[13];
        // skip packets 14-15 it is a timestamp used by the ps4
        state.battery = packet[16];

        let angular_velocity = Vec3 {
            x: f32::from_be_bytes([packet[17], packet[18], 0, 0]),
            y: f32::from_be_bytes([packet[19], packet[20], 0, 0]),
            z: f32::from_be_bytes([packet[21], packet[22], 0, 0]),
        };
        state.angular_velocity = angular_velocity;
    }
}
