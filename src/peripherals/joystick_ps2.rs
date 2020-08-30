use arduino_uno::prelude::*;
use arduino_uno::adc::Adc;
use arduino_uno::hal::port::{
    Pin,
    mode::{Analog, Input, Floating},
    portc::{PC0, PC1},
};

use crate::Direction;
use super::{InputDevice, InputSignal};


/// Object describing the input received from the JoyStick.
#[derive(Copy, Clone)]
pub struct JoyStickSignal {
    // Signed 8-bit integer where negative values indicate magnitude Left
    //   and positive values indicate magnitude Right.
    pub horiz: i8,
    // Signed 8-bit integer where negative values indicate magnitude Down
    //   and positive values indicate magnitude Up.
    pub vert: i8,
    // Boolean indicating if the button was pressed.
    pub button: bool,
}


impl JoyStickSignal {

    /// Convert the JoyStickSignal object into a single direction, if possible.
    /// 
    /// If no direction exceeds the threshold value, None value is returned.
    pub fn to_single_direction(self) -> Option<Direction> {
        if self.horiz.abs() > self.vert.abs() {
            if self.horiz < -JoyStick::THRESHOLD {
                return Some(Direction::Left)
            } else if self.horiz > JoyStick::THRESHOLD {
                return Some(Direction::Right)
            }
        }
        if self.vert < -JoyStick::THRESHOLD {
            return Some(Direction::Down)
        } else if self.vert > JoyStick::THRESHOLD {
            return Some(Direction::Up)
        }
        None
    }
}


/// Object that interfaces with the JoyStick peripheral.
pub struct JoyStick {
    // Analog pin that reads x-axis values.
    x_axis: PC0<Analog>,
    // Analog pin that reads y-axis values.
    y_axis: PC1<Analog>,
    // Digital pin that reads button presses.
    z_axis: Pin<Input<Floating>>,
}

impl JoyStick {
    const CENTER: i16 = 512;
    pub const THRESHOLD: i8 = 50;

    /// Creates a new JoyStick object.
    pub fn new(
        x_axis: PC0<Analog>,
        y_axis: PC1<Analog>,
        z_axis: Pin<Input<Floating>>,
    ) -> Self {
        JoyStick { x_axis, y_axis, z_axis }
    }
}


impl InputDevice for JoyStick {

    /// Read the input data from the JoyStick Peripheral.
    /// 
    /// # Arguments
    /// * adc - The Analog-Digital convertor required to read analog data.
    /// 
    /// # Returns
    /// Option<InputSignal::JoyStick>
    fn read(&mut self, adc: &mut Adc) -> Option<InputSignal> {
        let x: u16 = nb::block!(adc.read(&mut self.x_axis)).void_unwrap();
        let y: u16 = nb::block!(adc.read(&mut self.y_axis)).void_unwrap();
        let z: bool = self.z_axis.is_low().void_unwrap();
        let signal = JoyStickSignal {
            horiz: (((x as i16) - Self::CENTER) / 4) as i8,
            vert: (((y as i16) - Self::CENTER) / 4) as i8,
            button: z,
        };
        if (signal.button) | (signal.horiz.abs() > Self::THRESHOLD) | (signal.vert.abs() > Self::THRESHOLD) {
            return Some(InputSignal::JoyStick(signal))
        }
        None
    }
}