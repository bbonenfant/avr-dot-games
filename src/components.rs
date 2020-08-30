const BAUD_RATE: u32 = 9600;


/// The Peripheral components used for the AVR Dot Games.
pub struct Components {
    /// ADC used to read analog input values.
    adc: arduino_uno::adc::Adc,
    /// Random number generator.
    rng: crate::peripherals::XOrShiftPrng,
    /// The serial connection. Used for debugging purposes.
    pub serial: arduino_uno::Serial<arduino_uno::hal::port::mode::Floating>,
    /// The DotDisplay peripheral.
    pub display: crate::peripherals::DotDisplay,
    /// The JoyStick peripheral.
    pub joystick: crate::peripherals::InputPeripheral<crate::peripherals::JoyStick>,
}


/// Construct the `crate::Components` object.
pub fn get_components() -> Components {
    // Grab the peripheral pins.
    let dp = arduino_uno::Peripherals::take().unwrap();

    // Collect all the available pins.
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Create the ADC (Analog-to-Digital Converter) object.
    let mut adc = {
        let settings = arduino_uno::adc::AdcSettings::default();
        arduino_uno::adc::Adc::new(dp.ADC, settings)
    };

    // Create the (Pseudo) Random Number Generator.
    let rng = {
        let pin = pins.a5.into_analog_input(&mut adc);
        crate::peripherals::XOrShiftPrng::new(pin, &mut adc)
    };

    // Construct a Serial object (used for debugging purposes).
    let serial = {
        let rx = pins.d0;
        let tx = pins.d1.into_output(&mut pins.ddr);
        arduino_uno::Serial::new(dp.USART0, rx, tx, BAUD_RATE)
    };

    // Create the peripheral components.
    let display = crate::peripherals::DotDisplay::new(
        pins.d10.into_output(&mut pins.ddr).downgrade(),
        pins.d13.into_output(&mut pins.ddr).downgrade(),
        pins.d11.into_output(&mut pins.ddr).downgrade(),
    );
    
    let joystick = {
        let x_axis = pins.a0.into_analog_input(&mut adc);
        let y_axis = pins.a1.into_analog_input(&mut adc);
        let z_axis = pins.a2.into_floating_input(&mut pins.ddr).downgrade();
        crate::peripherals::InputPeripheral::new(
            crate::peripherals::JoyStick::new(x_axis, y_axis, z_axis)
        )
    };

    Components { adc, rng, serial, display, joystick }
}


/// Implement a RngCore as a pass through to the rng attribute.
/// 
/// This simplifies the user interface, removing the need to handle the ADC.
impl rand_core::RngCore for Components {

    /// Returns a pseudo-randomly generated u32 number.
    fn next_u32(&mut self) -> u32 {
        self.rng.generate(&mut self.adc) as u32
    }

    /// Returns a pseudo-randomly generated u64 number.
    fn next_u64(&mut self) -> u64 {
        self.rng.generate(&mut self.adc) as u64
    }

    /// Fill `dest` with random data.
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    /// Fill `dest` entirely with random data.
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}


impl Components {

    /// Pass through function to the [InputPeripheral.poll](peripherals/struct.InputPeripheral.html#method.poll)
    ///   method with type parameter [Joystick](peripherals/struct.JoyStick).
    /// 
    /// This simplifies the user interface, removing the need to handle the ADC.
    pub fn poll_joystick(&mut self, duration_ms: usize) -> &crate::peripherals::PollArray {
        self.joystick.poll(&mut self.adc, duration_ms)
    }

    /// Pass through function to the [InputPeripheral.poll](peripherals/struct.InputPeripheral.html#method.poll_until_any)
    ///   method with type parameter [Joystick](peripherals/struct.JoyStick).
    /// 
    /// This simplifies the user interface, removing the need to handle the ADC.
    pub fn poll_joystick_until_any(&mut self) -> crate::peripherals::InputSignal {
        self.joystick.poll_until_any(&mut self.adc)
    }
}