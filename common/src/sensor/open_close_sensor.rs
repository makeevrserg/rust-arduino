use embedded_hal::digital::InputPin;

pub struct OpenCloseSensorPin<P> {
    pin: P,
}

impl<P, E> OpenCloseSensorPin<P>
where
    P: InputPin<Error=E>,
{
    pub fn new(pin: P) -> impl OpenCloseSensor {
        Self { pin }
    }
}

pub trait OpenCloseSensor {
    fn is_open(&mut self) -> bool;
}


impl<P, E> OpenCloseSensor for OpenCloseSensorPin<P>
where
    P: InputPin<Error=E>,
{
    fn is_open(&mut self) -> bool {
        self.pin.is_high().unwrap_or(false)
    }
}
