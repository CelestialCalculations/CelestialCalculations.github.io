pub const DISTANCE_TOLERANCE: f64 = 0.1e-5;

pub fn is_close(expected_val: f64, real_val: f64) -> bool {
    (expected_val - real_val).abs() <= DISTANCE_TOLERANCE
}
