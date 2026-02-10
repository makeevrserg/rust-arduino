use core::f32::consts::PI;

const TWO_PI: f32 = 2.0 * PI;
const HALF_PI: f32 = PI / 2.0;
const PI_OVER_180: f32 = PI / 180.0;

/// Fast normalize using conditional subtraction instead of modulo
fn normalize_radians_fast(mut x: f32) -> f32 {
    // Bring into range [-2π, 2π] first
    if x > TWO_PI || x < -TWO_PI {
        // Use integer division via truncation (faster on AVR)
        let k = (x / TWO_PI) as i32;
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
