#![no_std]
#![no_main]

mod math;

use ufmt::{uWrite, uwriteln};
use math::math::{sin, cos};

use panic_halt as _;
use arduino_hal::{I2c, Usart};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Line,
    geometry::Point,
    Drawable,
};
use ssd1306::{
    prelude::*,
    I2CDisplayInterface,
    Ssd1306,
};

use arduino_hal::delay_ms;
use arduino_hal::hal::Atmega;
use arduino_hal::pac::USART0;
use embedded_graphics::primitives::PrimitiveStyle;


#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => {
        let _ = ufmt::uwriteln!(&mut $logger.serial, $($arg)*);
    };
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
        100_000,
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();


    let cx = (display.size().width / 2) as i32;
    let cy = (display.size().height / 2) as i32;
    let w = 30;
    let h = 10;

    let mut angle: i32 = 0; // milli-radians

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        display.clear(BinaryColor::Off).unwrap();

        let corners = [
            Point::new(cx - w / 2, cy - h / 2),
            Point::new(cx + w / 2, cy - h / 2),
            Point::new(cx + w / 2, cy + h / 2),
            Point::new(cx - w / 2, cy + h / 2),
        ];


        // rotate and draw
        for i in 0..4 {
            let p1 = corners[i];
            let p2 = corners[(i + 1) % 4];
            let p1r = rotate_point(p1, cx, cy, angle);
            let p2r = rotate_point(p2, cx, cy, angle);

            // Draw the line
            Line::new(p1r, p2r)
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(&mut display)
                .unwrap();
        }
        display.flush().unwrap();

        angle += 5; // Increment by 5 milli-radians

        const TWO_PI_MILLI_RAD: i32 = 6283; // 2π in milli-radians

        // Wrap the angle within 0..6283
        if angle > TWO_PI_MILLI_RAD {
            angle -= TWO_PI_MILLI_RAD;
        } else if angle < 0 {
            angle += TWO_PI_MILLI_RAD;
        }
        ufmt::uwriteln!(serial,"angle: {}",angle);

        delay_ms(1);
    }
}


fn rotate_point(p: Point, cx: i32, cy: i32, angle: i32) -> Point {
    const TWO_PI_MILLI_RAD: i32 = 6283; // 2π in milli-radians
    let angle = ((angle % TWO_PI_MILLI_RAD) + TWO_PI_MILLI_RAD) % TWO_PI_MILLI_RAD; // Ensure angle is in 0..6283

    // cos_a and sin_a are scaled by 256 (i.e., 1.0 = 256)
    let cos_a = cos(angle) as i64; // scaled by 256
    let sin_a = sin(angle) as i64; // scaled by 256

    // Translate point to origin relative to (cx, cy)
    let dx = (p.x as i64 - cx as i64);
    let dy = (p.y as i64 - cy as i64);

    // Perform rotation using fixed-point arithmetic
    let new_x = (dx * cos_a - dy * sin_a) >> 8; // Divide by 256 using right shift
    let new_y = (dx * sin_a + dy * cos_a) >> 8; // Divide by 256 using right shift

    // Translate back and clamp to the specified ranges
    let new_x = (new_x + cx as i64).clamp(0, 127);
    let new_y = (new_y + cy as i64).clamp(0, 31);

    Point::new(new_x as i32, new_y as i32)
}
