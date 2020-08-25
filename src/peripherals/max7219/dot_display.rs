use arduino_uno::prelude::*;
use arduino_uno::hal::port::{Pin, mode::Output};

use super::DotScreen;

/// The address of the register on the DotDisplay chip.
#[derive(Clone, Copy)]
#[repr(u8)]
enum RegisterAddress {
    Column1 = 0x1,
    Column2 = 0x2,
    Column3 = 0x3,
    Column4 = 0x4,
    Column5 = 0x5,
    Column6 = 0x6,
    Column7 = 0x7,
    Column8 = 0x8,
    Decode = 0x9,
    Intensity = 0xA,
    ScanLimit = 0xB,
    Shutdown = 0xC,
    Test = 0xF,
}

/// The object the interfaces with the MAX7219 8x8 LED Dot Display peripheral.
pub struct DotDisplay {
    // The chip select pin.
    cs: Pin<Output>,
    // The clock pin.
    clk: Pin<Output>,
    // The data input-output pin.
    dio: Pin<Output>,
}

impl DotDisplay {
    const COLUMNS: [RegisterAddress; 8] = [
        RegisterAddress::Column1, RegisterAddress::Column2, RegisterAddress::Column3, RegisterAddress::Column4,
        RegisterAddress::Column5, RegisterAddress::Column6, RegisterAddress::Column7, RegisterAddress::Column8,
    ];

    /// Create a new DotDisplay object.
    /// 
    /// # Arguments
    /// 
    /// * `chip_select_pin` - The pin used to select this DotDisplay.
    /// * `clock_pin`       - The pin used as the clock for the SPI data transfers.
    /// * `data_io_pin`     - The pin used to transmit data. 
    pub fn new(
        mut chip_select_pin: Pin<Output>,
        mut clock_pin: Pin<Output>,
        mut data_io_pin: Pin<Output>,
    ) -> Self {
        // Initialize the pin digital outputs.
        chip_select_pin.set_high().void_unwrap();
        clock_pin.set_low().void_unwrap();
        data_io_pin.set_low().void_unwrap();
        Self { cs: chip_select_pin, clk: clock_pin, dio: data_io_pin }.init()
    }

    /// Initialize the dot display by initializing data within its registers.
    /// 
    /// This includes:
    ///   * Turning the display on.
    ///   * Turning test-mode off.
    ///   * Turning decode-mode off.
    ///   * Enabling all columns.
    ///   * Clearing the display.
    fn init(mut self) -> Self {
        // Turn display on.
        self.shutdown(false);

        // Turn test-mode off.
        self.test(false);

        // Turn decode mode off.
        self.send_raw_data(RegisterAddress::Decode, 0);

        // Enable all columns.
        self.send_raw_data(RegisterAddress::ScanLimit, 7);

        // Set the intensity of the LEDs to bright, but not full intensity.
        self.set_intensity(12);

        // Clear display.
        self.clear();

        return self
    }

    /// Send raw data to the dot display over the SPI protocol.
    /// 
    /// The serial data format uses 16 bits:
    ///  | D15 | D14 | D13 | D12 | D11 | D10 | D09 | D08 | D07 | D06 | D05 | D04 | D03 | D02 | D01 | D00 |
    /// where 
    ///   * D11-D08 describe the register address to write a command, 
    ///   * D07-D00 is the command data,
    ///   * D15-D12 are "don't care" bits.
    /// The data is expected in MSB order.
    /// Due to the nature of how data is written to the device, 
    ///   only 12 bits of data needs to be written for each serial message, 
    ///   where D15-D12 are skipped over.
    /// 
    /// # Arguments
    /// 
    /// * `register` - A RegisterAddress object corresponding to the register 
    ///                 address on the device to write the command. 
    /// * `data`     - The data of the command.
    fn send_raw_data(&mut self, register: RegisterAddress, data: u8) {
        let message = ((register as u16) << 8) | data as u16;
        self.cs.set_low().void_unwrap();
        (4..16).for_each(|shift| {
            if (message & (1 << 15 - shift)) != 0 { 
                self.dio.set_high().void_unwrap() 
            } else { 
                self.dio.set_low().void_unwrap() 
            }
            self.clk.set_high().void_unwrap();
            self.clk.set_low().void_unwrap();
        });
        self.cs.set_high().void_unwrap();
        self.dio.set_low().void_unwrap();
    }
    
    /// Print a DotScreen to the display
    pub fn show(&mut self, screen: &DotScreen) {
        for (&col, &data) in Self::COLUMNS.iter().zip(screen.columns.iter()) {
            self.send_raw_data(col, data);
        }
    }

    /// Turn off all the LED lights of the display.
    pub fn clear(&mut self) {
        Self::COLUMNS.iter().for_each(|&col| {
            self.send_raw_data( col, 0b00000000);
        });
    }

    /// Set the intensity of the LED lights. 
    /// 
    /// # Arguments
    /// 
    /// * `level` - The level of intensity of the LED lights.
    ///             This varies from 0 (lowest) to 15 (highest).
    ///             Supplying a level outside this range is undefined.
    pub fn set_intensity(&mut self, level: u8) {
        self.send_raw_data(RegisterAddress::Intensity, level);
    }

    /// Shutdown the display.
    /// 
    /// This turns the LED lights off but does not overwrite the data for each LED.
    pub fn shutdown(&mut self, off: bool) {
        self.send_raw_data(RegisterAddress::Shutdown, !off as u8);
    }

    /// Enables test-mode for the display.
    /// 
    /// This turns on all LED lights at full intensity. This does no overwrite the
    ///   data for each LED. This has precedence over "shutdown" mode.
    pub fn test(&mut self, on: bool) {
        self.send_raw_data(RegisterAddress::Test, on as u8);
    }
}


