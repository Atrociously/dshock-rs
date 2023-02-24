mod flags;
mod mode;

use std::{ops::Deref, marker::PhantomData};

use flags::flags;
use hidapi::{HidError, HidApi, HidDevice};
use mode::Mode;

const DS4_VID: u16 = 0x054C;
const DS4_PID: u16 = 0x09CC;

flags!(Buttons {
    PS = 0x40,
    OPTION = 0x20,
    SHARE = 0x10,
    TRIANGLE = 0x08,
    CIRCLE = 0x04,
    CROSS = 0x02,
    SQUARE = 0x01,
});

flags!(DPad {
    UP = 0x08,
    RIGHT = 0x04,
    DOWN = 0x02,
    LEFT = 0x01,
});

flags!(Triggers {
    R2 = 0x08,
    L2 = 0x04,
    R1 = 0x02,
    L1 = 0x01,
});

flags!(Sticks {
    RSTICK = 0x02,
    LSTICK = 0x01,
});

impl From<u8> for Buttons {
    fn from(value: u8) -> Self {
        Self(value & 0x7F)
    }
}

impl From<u8> for DPad {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0 => DPad::UP,
            1 => DPad::UP | DPad::RIGHT,
            2 => DPad::RIGHT,
            3 => DPad::DOWN | DPad::RIGHT,
            4 => DPad::DOWN,
            5 => DPad::DOWN | DPad::LEFT,
            6 => DPad::LEFT,
            7 => DPad::UP | DPad::LEFT,
            8 => DPad::default(),
            _ => unreachable!("invalid controller response")
        }
    }
}

impl From<u8> for Triggers {
    fn from(value: u8) -> Self {
        Self(value & 0xF)
    }
}

impl From<u8> for Sticks {
    fn from(value: u8) -> Self {
        Self(value & 0x03)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Position of sticks
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

pub struct Controller<M> {
    device: HidDevice,
    state: Box<ControllerState>,
    _mode: PhantomData<M>,
}

#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct ControllerState {
    pub stick_left: Pos,
    pub stick_right: Pos,
    pub direction: DPad,
    pub buttons: Buttons,
    pub triggers: Triggers,
    pub sticks: Sticks,
    pub trigger_z_left: u8,
    pub trigger_z_right: u8,
    pub battery: u8,
    pub angular_velocity: Vec3,
    pub acceleration: Vec3,
}

impl<M: Mode> Controller<M> {
    pub fn new(_: M) -> Result<Self, HidError> {
        let device = HidApi::new()?.open(DS4_VID, DS4_PID)?;
        let state = Box::<ControllerState>::default();

        Ok(Self {
            device,
            state,
            _mode: PhantomData,
        })
    }

    pub fn update(&mut self) -> Result<(), HidError> {
        // read the whole 64 byte hid device report packet
        let packet = {
            let mut packet = [0u8; 64];
            self.device.read(&mut packet)?;
            packet
        };
        M::parse(&mut self.state, packet);
        Ok(())
    }
}

impl<M> std::fmt::Debug for Controller<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.state.fmt(f)
    }
}

impl<M> Deref for Controller<M> {
    type Target = ControllerState;

    fn deref(&self) -> &Self::Target {
        self.state.deref()
    }
}
