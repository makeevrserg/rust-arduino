use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::Ssd1306;
use crate::renderer::Flushable;

impl<DI, SIZE> Flushable for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: ssd1306::prelude::WriteOnlyDataCommand,
    SIZE: ssd1306::size::DisplaySize,
{
    type Error = ();

    fn flush(&mut self) -> Result<(), Self::Error> {
        // Call Ssd1306's flush method directly
        Ssd1306::flush(self).map_err(|_| ())
    }
}
