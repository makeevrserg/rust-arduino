#![no_std]

// No std needed - we're using core only
use core::f32::consts::PI;

// Precomputed values for faster calculations
const TWO_PI: f32 = 2.0 * PI;
const HALF_PI: f32 = PI / 2.0;
const PI_OVER_180: f32 = PI / 180.0;
const RECIP_TWO_PI: f32 = 1.0 / (2.0 * PI); // For faster modulo

/// Fast normalize using conditional subtraction instead of modulo
fn normalize_radians_fast(mut x: f32) -> f32 {
    // Bring into range [-2π, 2π] first
    if x > TWO_PI || x < -TWO_PI {
        // Use integer division via truncation (faster on AVR)
        let mut k = (x / TWO_PI) as i32;
        x -= (k as f32) * TWO_PI;
    }

    // Now reduce to [-π, π]
    if x > PI {
        x - TWO_PI
    } else if x < -PI {
        x + TWO_PI
    } else {
        x
    }
}

/// Fast degrees to radians using precomputed constant
fn deg_to_rad_fast(deg: f32) -> f32 {
    deg * PI_OVER_180
}

/// Optimized sine using fewer terms and better normalization
pub fn sin_fast(deg: f32) -> f32 {
    // Convert and normalize
    let mut x = deg_to_rad_fast(deg);
    x = normalize_radians_fast(x);

    // Use sine approximation for smaller range [-π/2, π/2]
    // If x is in [π/2, 3π/2], use cos(x - π/2) symmetry
    if x > HALF_PI {
        return cos_fast_raw(x - HALF_PI);
    } else if x < -HALF_PI {
        return -cos_fast_raw(x + HALF_PI);
    }

    // Use polynomial approximation for sin(x) on [-π/2, π/2]
    // 5th order minimax polynomial approximation (good enough for most applications)
    let x2 = x * x;
    // Coefficients for sin(x) ~ x + a*x³ + b*x⁵
    let result = x * (1.0 - x2 * (1.0/6.0 - x2 * (1.0/120.0)));
    result
}

/// Helper function for cosine approximation
fn cos_fast_raw(x: f32) -> f32 {
    // Use polynomial approximation for cos(x) on [-π/2, π/2]
    let x2 = x * x;
    // Coefficients for cos(x) ~ 1 + a*x² + b*x⁴
    1.0 - x2 * (0.5 - x2 * (1.0/24.0))
}

/// Fast cosine using symmetry with sine
pub fn cos_fast(deg: f32) -> f32 {
    // cos(x) = sin(x + π/2)
    sin_fast(deg + 90.0)
}

/// Alternative: Direct cosine with normalization
pub fn cos_fast_direct(deg: f32) -> f32 {
    let mut x = deg_to_rad_fast(deg);
    x = normalize_radians_fast(x);

    // Use cosine approximation for smaller range [-π/2, π/2]
    // If x is in [π/2, 3π/2], use -cos(x - π) symmetry
    if x > HALF_PI {
        return -cos_fast_raw(x - PI);
    } else if x < -HALF_PI {
        return -cos_fast_raw(x + PI);
    }

    cos_fast_raw(x)
}

/// EVEN FASTER: Lookup table with linear interpolation (most efficient for fixed-point inputs)
/// Use this if your angles are integers or have limited precision
pub fn sin_lookup(deg_int: i16) -> f32 {
    // Create a lookup table for 0-90 degrees (quarter sine)
    static SIN_TABLE: [u8; 91] = [
        0, 2, 4, 6, 8, 10, 12, 14, 16, 18,  // 0-9
        20, 22, 24, 25, 27, 29, 31, 33, 35, 37,  // 10-19
        39, 41, 42, 44, 46, 48, 49, 51, 53, 54,  // 20-29
        56, 57, 59, 61, 62, 64, 65, 66, 68, 69,  // 30-39
        71, 72, 73, 74, 76, 77, 78, 79, 80, 81,  // 40-49
        82, 83, 84, 85, 86, 87, 88, 89, 89, 90,  // 50-59
        91, 92, 92, 93, 94, 94, 95, 95, 96, 96,  // 60-69
        97, 97, 97, 98, 98, 98, 99, 99, 99, 99,  // 70-79
        99, 99, 100, 100, 100, 100, 100, 100, 100, 100,  // 80-89
        100  // 90
    ];

    // Normalize angle to 0-359
    let mut angle = deg_int % 360;
    if angle < 0 { angle += 360; }

    let quarter = angle / 90;
    let remainder = angle % 90;

    // Get base value from table
    let base = SIN_TABLE[remainder as usize] as f32 / 100.0;
    let next = SIN_TABLE[(remainder + 1) as usize % 91] as f32 / 100.0;

    // Linear interpolation
    let value = base + (next - base) * ((angle % 90) as f32 / 90.0);

    // Apply quadrant corrections
    match quarter {
        0 => value,           // 0-89: sin positive
        1 => 1.0 - value,     // 90-179: symmetric
        2 => -value,          // 180-269: negative
        _ => value - 1.0,     // 270-359: symmetric negative
    }
}

pub fn cos_lookup(deg_int: i16) -> f32 {
    // cos(x) = sin(x + 90)
    sin_lookup(deg_int + 90)
}