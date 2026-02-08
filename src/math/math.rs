#![no_std]

use core::f32::consts::PI;

/// Normalize angle in radians to range [-PI, PI]
fn normalize_radians(mut x: f32) -> f32 {
    let two_pi = 2.0 * PI;
    x = x % two_pi;
    if x > PI {
        x -= two_pi;
    } else if x < -PI {
        x += two_pi;
    }
    x
}

/// Convert degrees to radians
fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

/// Sine using Taylor series
pub fn sin(deg: f32) -> f32 {
    let mut x = deg_to_rad(deg);
    x = normalize_radians(x);

    let mut term = x; // first term
    let mut sum = x;

    // Taylor series: sin(x) = x - x³/3! + x⁵/5! - ...
    for i in 1..10 {
        let n = (2 * i) as f32;
        term *= -x * x / (n * (n + 1.0));
        sum += term;
    }

    sum
}

/// Cosine using Taylor series
pub fn cos(deg: f32) -> f32 {
    let mut x = deg_to_rad(deg);
    x = normalize_radians(x);

    let mut term = 1.0;
    let mut sum = 1.0;

    // Taylor series: cos(x) = 1 - x²/2! + x⁴/4! - ...
    for i in 1..10 {
        let n = (2 * i - 1) as f32;
        term *= -x * x / (n * (n + 1.0));
        sum += term;
    }

    sum
}
