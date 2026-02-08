#![no_std]
#![no_main]

mod math;
mod geometry;

use ufmt::uWrite;

use arduino_hal::I2c;
use embedded_graphics::{
    geometry::Point,
    pixelcolor::BinaryColor,
    Drawable,
};
use ssd1306::{
    I2CDisplayInterface,
    Ssd1306,
};

use crate::geometry::geometry::Square;
use arduino_hal::delay_ms;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use ssd1306::mode::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => {
        let _ = ufmt::uwriteln!(&mut $logger.serial, $($arg)*);
    };
}


#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    // I2C on Nano: SDA = A4, SCL = A5
    let i2c = I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        600_000,
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();


    let center = Point::new((display.size().width / 2) as i32, (display.size().height / 2) as i32);
    let mut angle: i32 = 0; // milli-radians

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        display.clear(BinaryColor::Off).unwrap();


        let square = Square::new(30, center);
        let square_rotated = square.rotate(angle, center);
        let corners = square_rotated.corners();
        for i in 0..corners.iter().len() {
            let p1 = corners[i];
            let p2 = corners[(i + 1) % 4];

            Line::new(p1, p2)
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(&mut display)
                .unwrap();
        }


        display.flush().unwrap();

        angle += 45; // Increment by 5 milli-radians

        ufmt::uwriteln!(serial,"angle: {}",angle);

        delay_ms(1);
    }
}

