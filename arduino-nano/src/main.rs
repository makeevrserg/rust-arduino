#![no_std]
#![no_main]
#![allow(dead_code)]

use sensors::logger::logger::Logger;
use arduino_hal::I2c;
use graphics::component::{PulsatingCircle, RotatingSquare};
use graphics::renderer::embedded_graphics::renderer_impl::EmbeddedGraphicsAdapter;
use graphics::renderer::{Component, Renderer, Updatable};
use sensors::sensor::led_sensor::{LedSensor, LedSensorPin};
use sensors::sensor::open_close_sensor::{OpenCloseSensor, OpenCloseSensorPin};
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};
use sensors::log;
use sensors::logger::ufmt_logger::UWriteLoggable;

#[cfg(not(doc))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut logger = UWriteLoggable::new(serial);

    logger.log("Firmware panic!\r");

    if let Some(loc) = info.location() {
        log!(logger, "  At {}:{}:{}\r", loc.file(), loc.line(), loc.column());
    }
    let mut led = LedSensorPin::new(pins.d13.into_output());
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut logger = UWriteLoggable::new(serial);
    logger.log("#main started");
    // I2C on Nano: SDA = A4, SCL = A5
    let i2c = I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        600_000,
    );
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    logger.log("#main display initialized");
    let mut renderer = EmbeddedGraphicsAdapter::new(&mut display);

    let mut led = LedSensorPin::new(pins.d13.into_output());
    let mut door_sensor = OpenCloseSensorPin::new(pins.d2.into_pull_up_input());

    let center = renderer.canvas().center();

    let mut rotating_square = RotatingSquare::new(center, 30).with_rotation_step(45);
    let mut pulsating_circle = PulsatingCircle::new(center, 0, 16)
        .with_color(true)
        .with_filled(true);

    logger.log("#main entering loop");
    loop {
        renderer.clear(false);

        rotating_square.update();
        rotating_square.draw(&mut renderer);

        pulsating_circle.update();
        pulsating_circle.draw(&mut renderer);

        renderer.flush();

        if door_sensor.is_open() {
            led.turn_on();
        } else {
            led.turn_off();
        }

        // let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        // Text::new("Arduino Nano", Point::new(10, 17), text_style)
        //     .draw(&mut display)
        //     .unwrap();
        arduino_hal::delay_ms(10);
    }
}
