use ufmt::uWrite;
use crate::logger::logger::Logger;

pub struct UWriteLoggable<T: ufmt::uWrite> {
    serial: T,
}

impl<T: ufmt::uWrite> UWriteLoggable<T> {
    pub fn new(serial: T) -> impl Logger<T> {
        Self { serial }
    }
}

impl<T: uWrite> Logger<T> for UWriteLoggable<T> {
    fn log(&mut self, msg: &str) {
        ufmt::uwriteln!(self.serial, "{}", msg).unwrap_or_else(|never| match never { _ => {} });
    }

    fn serial(&mut self) -> &mut T {
        &mut self.serial
    }
}