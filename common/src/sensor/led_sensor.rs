use avr_hal_generic::port::{mode, PinOps};
use avr_hal_generic::port::Pin;

pub struct LedSensorPin<PIN: PinOps> {
    pin: Pin<mode::Output, PIN>,
}

impl<PIN: PinOps> LedSensorPin<PIN> {
    pub fn new(pin: Pin<mode::Output, PIN>) -> impl LedSensor {
        Self { pin }
    }
}

pub trait LedSensor {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
}

impl<PIN: PinOps> LedSensor for LedSensorPin<PIN>
{
    fn turn_on(&mut self) {
        self.pin.set_high();
    }


    fn turn_off(&mut self) {
        self.pin.set_low();
    }

    fn toggle(&mut self) {
        self.pin.toggle();
    }
}
