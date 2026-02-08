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
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    text::{Text, Baseline},
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
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();


    let cx = (display.size().width / 2) as i32;
    let cy = (display.size().height / 2) as i32;
    let w = 30;
    let h = 30;

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

        angle += 180; // Increment by 5 milli-radians

        ufmt::uwriteln!(serial,"angle: {}",angle);

        delay_ms(1);
    }
}


fn rotate_point(p: Point, cx: i32, cy: i32, angle: i32) -> Point {
    // Convert milli-radians to degrees (1000 milli-radians = 1 radian)
    let angle_rad = (angle as f32) / 1000.0;
    let angle_deg = angle_rad * 180.0 / core::f32::consts::PI;
    
    // Translate point to origin
    let x = (p.x - cx) as f32;
    let y = (p.y - cy) as f32;
    
    // Apply rotation matrix
    let cos_a = cos(angle_deg);
    let sin_a = sin(angle_deg);
    
    let xr = x * cos_a - y * sin_a;
    let yr = x * sin_a + y * cos_a;
    
    // Translate back and round to i32
    Point::new(
        (xr + cx as f32) as i32,
        (yr + cy as f32) as i32,
    )
}
