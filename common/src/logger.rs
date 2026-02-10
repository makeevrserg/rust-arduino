use core::cell::UnsafeCell;

pub struct Loggable<T> {
    serial: T,
}

impl<T> Loggable<T> where T: ufmt::uWrite {
    pub fn new(serial: T) -> impl Logger {
        Self { serial }
    }
}

pub trait Logger {
    fn log(self, msg: &str);
}

impl<T: ufmt::uWrite> Logger for Loggable<T> {
    fn log(mut self, msg: &str) {
        let _ = ufmt::uwriteln!(self.serial, "{}", msg);
    }
}
