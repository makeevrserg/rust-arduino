#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::I2c;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use ssd1306::{
    prelude::*,
    I2CDisplayInterface,
    Ssd1306,
};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // I2C on Nano: SDA = A4, SCL = A5
    let i2c = I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        100_000, // 400 kHz fast mode
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();

    // Draw a rectangle
    Rectangle::new(Point::new(0, 0), Size::new(127, 31))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw text
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    Text::new("Hello, Rust!", Point::new(10, 10), text_style)
        .draw(&mut display)
        .unwrap();

    Text::new("Arduino Nano", Point::new(10, 17), text_style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}
