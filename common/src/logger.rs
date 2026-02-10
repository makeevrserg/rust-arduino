
pub struct Loggable<T> {
    serial: T,
}

impl<T> Loggable<T> {
    pub fn new(serial: T) -> Self {
        Self { serial }
    }
}

pub trait Logger {
    fn log(&mut self, msg: &str);
}

impl<T: ufmt::uWrite> Logger for Loggable<T> {
    fn log(&mut self, msg: &str) {
        let _ = ufmt::uwriteln!(self.serial, "{}", msg);
    }
}