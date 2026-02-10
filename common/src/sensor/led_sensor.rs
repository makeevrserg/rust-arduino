use embedded_hal::digital::OutputPin;
pub struct LedSensorPin<P> {
    pin: P,
}

impl<P, E> LedSensorPin<P>
where
    P: OutputPin<Error = E>,
{
    pub fn new(pin: P) -> impl LedSensor {
        Self { pin }
    }
}

pub trait LedSensor {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}

impl<P, E> LedSensor for LedSensorPin<P>
where
    P: OutputPin<Error = E>,
{
    fn turn_on(&mut self) {
        let _ = self.pin.set_high();
    }


    fn turn_off(&mut self) {
        let _ = self.pin.set_low();
    }
}
