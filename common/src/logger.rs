use ufmt::uWrite;

pub struct Loggable<T> {
    serial: T,
}

impl<T: uWrite> Loggable<T> {
    pub fn new(serial: T) -> impl Logger<T> {
        Self { serial }
    }
}

pub trait Logger<T> {
    fn log(&mut self, msg: &str);
    fn serial(&mut self) -> &mut T;
}

impl<T: uWrite> Logger<T> for Loggable<T> {
    fn log(&mut self, msg: &str) {
        ufmt::uwriteln!(self.serial, "{}", msg).unwrap_or_else(|never| match never { _ => {} });
    }

    fn serial(&mut self) -> &mut T {
        &mut self.serial
    }
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $fmt:literal $($arg:tt)*) => {
        ufmt::uwriteln!($logger.serial(), $fmt $($arg)*).unwrap_or_else(|never| match never { _ => {} })
    };
}
