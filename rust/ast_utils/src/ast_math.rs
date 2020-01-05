const EPS: f64 = 0.000_000_05;

/// Returns the fractional part of a number
/// # Arguments
/// * x: f64
///
/// # Example:
/// ```
/// use ast_utils::ast_math::frac;
///
/// frac(1.5_f64); // 0.5
/// frac(-1.5_f64); // 0.5
/// ```
pub fn frac(x: f64) -> f64 {
    x.abs() - x.trunc().abs()
}

/// Determine if the number x is close to number y
/// # Arguments
/// * x: f64
/// * y: f64
///
/// # Example:
/// ```
/// use ast_utils::ast_math::is_close;
///
/// assert_eq!(true, is_close(0.5, 0.5));
/// ```
pub fn is_close(x: f64, y: f64) -> bool {
    (x - y).abs() <= EPS
}

/// rounds a floot upto scale places after comma
pub fn floor(value: f64, scale: u8) -> f64 {
    let multiplier = 10i64.pow(scale as u32) as f64;
    (value * multiplier).floor() / multiplier
}
