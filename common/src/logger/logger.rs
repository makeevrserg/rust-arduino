pub trait Logger<T> {
    fn log(&mut self, msg: &str);
    fn serial(&mut self) -> &mut T;
}



#[macro_export]
macro_rules! log {
    ($logger:expr, $fmt:literal $($arg:tt)*) => {
        ufmt::uwriteln!($logger.serial(), $fmt $($arg)*).unwrap_or_else(|never| match never { _ => {} })
    };
}
