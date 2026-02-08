#![no_std]
#![no_main]
#![allow(dead_code)]

mod component;
mod math;
mod renderer;

use arduino_hal::I2c;
use ssd1306::{
    mode::DisplayConfig,
    rotation::DisplayRotation,
    size::DisplaySize128x64,
    I2CDisplayInterface,
    Ssd1306,
};
use crate::component::Square;
use crate::renderer::{Component, Renderer};
use renderer::embedded_graphics::renderer_impl::EmbeddedGraphicsAdapter;
use arduino_hal::delay_ms;
use crate::component::circle::Circle;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let _serial = arduino_hal::default_serial!(dp, pins, 57600);

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
    let mut renderer = EmbeddedGraphicsAdapter::new(&mut display);

    let mut led = pins.d13.into_output();

    let mut angle: i32 = 0;
    let mut circle_radius = 3;
    let mut circle_sign = 1;

    loop {
        renderer.clear(false);
        let center = renderer.canvas().center();

        let square = Square::new(center, 30).with_rotation(angle);
        square.draw(&mut renderer);

        let circle = Circle::new(center, circle_radius, true, true);
        circle.draw(&mut renderer);

        renderer.flush();

        angle = (angle + 45) % 6283;

        if (circle_radius <= 0) { circle_sign = 1; } else if (circle_radius >= 16) { circle_sign = -1; }
        circle_radius = circle_radius + circle_sign;

        led.toggle();

        delay_ms(10);
    }
}
