const BAUD_RATE: u32 = 9600;

/// The Peripheral components used for the AVR Dot Games.
pub struct Components {
    /// The serial connection. Used for debugging purposes.
    pub serial: arduino_uno::Serial<arduino_uno::hal::port::mode::Floating>,
    // The DotDisplay peripheral.
    pub display: crate::peripherals::DotDisplay,
    // The JoyStick peripheral.
    pub joystick: crate::peripherals::InputPeripheral<crate::peripherals::JoyStick>,
}


/// Construct the `crate::Components` object.
pub fn get_components() -> Components {
    // Grab the peripheral pins.
    let dp = arduino_uno::Peripherals::take().unwrap();

    // Collect all the available pins.
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

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
        // Create the ADC (Analog-to-Digital Converter) object used to read analog input from the JoyStick.
        let mut adc = {
            let settings = arduino_uno::adc::AdcSettings::default();
            arduino_uno::adc::Adc::new(dp.ADC, settings)
        };
        let x_axis = pins.a0.into_analog_input(&mut adc);
        let y_axis = pins.a1.into_analog_input(&mut adc);
        let z_axis = pins.a2.into_floating_input(&mut pins.ddr).downgrade();
        crate::peripherals::InputPeripheral::new(
            crate::peripherals::JoyStick::new(x_axis, y_axis, z_axis, adc)
        )
    };

    Components { serial, display, joystick }
}