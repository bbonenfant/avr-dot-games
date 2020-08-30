/// Functionality having to do with receiving "InputSignals" from peripherals.
use arduino_uno::adc::Adc;
use super::JoyStickSignal;


/// An enumeration of the possible "InputSignals".
pub enum InputSignal {
    JoyStick(JoyStickSignal)
}


/// This trait signifies that the peripheral device can read "InputSignals".
pub trait InputDevice {

    /// Read input data.
    /// 
    /// # Arguments
    /// * adc - The Analog-Digital convertor required to read analog data.
    fn read(&mut self, adc: &mut Adc) -> Option<InputSignal>;
}


/// This PollArray is used to record a sequence of InputSignals over a period of time.
pub type PollArray = arraydeque::ArrayDeque<[InputSignal; 100], arraydeque::Wrapping>;

/// This struct wraps an InputDevice, providing functionality for reading streams
///   of "InputSignals".
pub struct InputPeripheral<D: InputDevice> {
    device: D,
    deque: PollArray,
}

impl<D> InputPeripheral<D>
  where D: InputDevice 
{
    const POLL_DELAY_US: u16 = 950;

    /// Construct a new InputPeripheral.
    pub fn new(device: D) -> Self {
        Self { device, deque: arraydeque::ArrayDeque::new() }
    }

    /// Poll the InputDevice, collecting data a period of time.
    /// 
    /// # Arguments
    /// * adc         - The Analog-Digital convertor required to read analog data.
    /// * duration_ms - The duration of time, in milliseconds, over which
    ///                   to poll the InputDevice.
    /// 
    /// # Returns
    /// Reference to the PollArray object that recorded all "InputSignals"
    ///   from the InputDevice.
    pub fn poll(&mut self, adc: &mut Adc, duration_ms: usize) -> &PollArray {
        self.deque.clear();
        (0..duration_ms).for_each(|_| {
            if let Some(signal) = self.device.read(adc) {
                self.deque.push_back(signal);
            };
            arduino_uno::delay_us(Self::POLL_DELAY_US);
        });
        &self.deque
    }

    /// Poll the InputDevice continuously until any "InputSignal" is received.
    ///
    /// # Arguments
    /// * adc - The Analog-Digital convertor required to read analog data.
    /// 
    /// # Returns
    /// The first "InputSignal" received from the device.
    pub fn poll_until_any(&mut self, adc: &mut Adc) -> InputSignal {
        loop {
            if let Some(signal) = self.device.read(adc) {
                return signal
            }
            arduino_uno::delay_us(Self::POLL_DELAY_US);
        }
    }
}