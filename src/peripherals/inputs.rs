/// Functionality having to do with receiving "InputSignals" from peripherals.
use super::JoyStickSignal;


/// An enumeration of the possible "InputSignals".
pub enum InputSignal {
    JoyStick(JoyStickSignal)
}


/// This trait signifies that the peripheral device can read "InputSignals".
pub trait InputDevice {

    /// Read input data.
    fn read(&mut self) -> Option<InputSignal>;
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
    /// * duration_ms - The duration of time, in milliseconds, over which
    ///                   to poll the InputDevice.
    /// 
    /// # Returns
    /// Reference to the PollArray object that recorded all "InputSignals"
    ///   from the InputDevice.
    pub fn poll(&mut self, duration_ms: usize) -> &PollArray {
        self.deque.clear();
        (0..duration_ms).for_each(|_| {
            if let Some(signal) = self.device.read() {
                self.deque.push_back(signal);
            };
            arduino_uno::delay_us(Self::POLL_DELAY_US);
        });
        &self.deque
    }

    /// Poll the InputDevice continuously until any "InputSignal" is received.
    ///
    /// # Returns
    /// The first "InputSignal" received from the device.
    pub fn poll_until_any(&mut self) -> InputSignal {
        loop {
            if let Some(signal) = self.device.read() {
                return signal
            }
            arduino_uno::delay_us(Self::POLL_DELAY_US);
        }
    }
}