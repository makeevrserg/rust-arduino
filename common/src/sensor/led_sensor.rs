use embedded_hal::digital::{PinState, StatefulOutputPin};

pub struct LedSensorPin<P> {
    pin: P,
}

impl<P, E> LedSensorPin<P>
where
    P: StatefulOutputPin<Error=E>,
{
    pub fn new(pin: P) -> impl LedSensor {
        Self { pin }
    }
}

pub trait LedSensor {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
}

impl<P, E> LedSensor for LedSensorPin<P>
where
    P: StatefulOutputPin<Error=E>,
{
    fn turn_on(&mut self) {
        self.pin.set_high().unwrap_or_default();
    }


    fn turn_off(&mut self) {
        self.pin.set_low().unwrap_or_default();
    }

    fn toggle(&mut self) {
        let was_low: bool = self.pin.is_set_low().unwrap_or(true);
        self.pin.set_state(PinState::from(was_low)).unwrap_or_default();
    }
}
