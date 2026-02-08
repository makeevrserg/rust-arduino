pub trait Flushable {
    type Error;
    fn flush(&mut self) -> Result<(), Self::Error>;
}
