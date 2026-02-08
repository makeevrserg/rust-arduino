/// Calculates sine of an angle in milli-radians using Taylor series approximation
/// Returns a fixed-point value scaled by 256 (i.e., 1.0 = 256)
pub fn sin(angle: i32) -> i32 {
    const SCALE: i64 = 256;
    const TWO_PI: i64 = 6283; // 2π in milli-radians
    let x = ((angle as i64 % TWO_PI) + TWO_PI) % TWO_PI;
    let x_scaled = x * SCALE;
    let x2 = (x_scaled * x) / SCALE;
    let x3 = (x2 * x) / SCALE;
    let x5 = (x3 * x2) / SCALE;
    let term1 = x_scaled;
    let term2 = x3 / 6;
    let term3 = x5 / 120;
    ((term1 - term2 + term3) / SCALE).clamp(-256, 255) as i32
}

/// Calculates cosine of an angle in milli-radians using phase-shifted sine
/// Returns a fixed-point value scaled by 256 (i.e., 1.0 = 256)
pub fn cos(angle: i32) -> i32 {
    sin(angle + 1571) // π/2 in milli-radians
}
