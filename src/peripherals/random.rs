use arduino_uno::prelude::*;
use arduino_uno::adc::Adc;
use arduino_uno::hal::port::{
    mode::Analog,
    portc::PC5,
};


pub type RngType = usize;

/// Implementation of a sufficiently-random Pseudo Random Number Generator 
///   that utilizes an ADC.
pub struct XOrShiftPrng {
    /// The current random number.
    bits: RngType,
    /// The analog pin from which to read. This pin is expected to be floating.
    pin: PC5<Analog>,
}

impl XOrShiftPrng {

    const BIT_COUNT: usize = RngType::MIN.count_zeros() as usize;

    /// Create and initialize a new XOrShiftPrng object.
    pub fn new(pin: PC5<Analog>, adc: &mut Adc) -> Self {
        let mut rng = Self { bits: 0, pin };
        rng.shuffle(adc);
        rng
    }

    /// Shuffle the bits, i.e. generate a new random number.
    /// 
    /// This is done by successively reading the 8 least significant bits
    ///   from a designated analog input and XOR-ing it with the current random
    ///   number, while with each read rotating the random number's bits to the
    ///   left. This is done for each bit of the RngType type.
    fn shuffle(&mut self, adc: &mut Adc) {
        for _ in 0..Self::BIT_COUNT {
            let sample: u16 = nb::block!(adc.read(&mut self.pin)).void_unwrap();
            self.bits = self.bits.rotate_left(1) ^ ((sample & 255) as RngType);
        }
    }

    /// Generate a random (ish) RngType number.
    /// 
    /// # Arguments
    /// * adc - The Analog-Digital convertor required to read analog data.
    pub fn generate(&mut self, adc: &mut Adc) -> RngType {
        self.shuffle(adc);
        self.bits.clone()
    }
}
