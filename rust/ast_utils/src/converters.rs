use std::fmt;

#[derive(Debug)]
pub enum DistanceUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
    Inch,
    Feet,
    Mile,
    LightYear,
    AstronomicalUnit,
    Parsec,
}

impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enum_label: &str = match self {
            DistanceUnit::Millimeter => "millimeter",
            DistanceUnit::Centimeter => "centimeter",
            DistanceUnit::Meter => "meter",
            DistanceUnit::Kilometer => "kilometer",
            DistanceUnit::Inch => "inch",
            DistanceUnit::Feet => "feet",
            DistanceUnit::Mile => "mile",
            DistanceUnit::LightYear => "light year",
            DistanceUnit::AstronomicalUnit => "astronomical unit",
            DistanceUnit::Parsec => "parsec",
            _ => "unspecified",
        };

        write!(f, "{}", enum_label)
    }
}

trait MetricScale<T: std::ops::Mul> {
    fn pow10(&self, power: i32) -> T;
}

impl MetricScale<f64> for f64 {
    fn pow10(&self, n: i32) -> f64 {
        let scale = (10.0 as f64).powi(n);
        self * scale
    }
}

trait MetricConvertable {
    fn to_metric(&self, to_unit: DistanceUnit) -> Result<f64, &str>;
}

#[derive(Debug)]
struct Distance {
    value: f64,
    unit: DistanceUnit,
}

impl Distance {
    pub fn new(value: f64, unit: DistanceUnit) -> Self {
        Distance { value, unit }
    }

    pub fn unit(&self) -> &DistanceUnit {
        &self.unit
    }
}

impl MetricConvertable for Distance {
    fn to_metric(&self, to_unit: DistanceUnit) -> Result<f64, &str> {
        let value = self.value;
        if value == 0.0 {
            return Ok(value);
        }

        match self.unit {
            DistanceUnit::Millimeter => match to_unit {
                DistanceUnit::Millimeter => Ok(value),
                DistanceUnit::Centimeter => Ok(value.pow10(-1)),
                DistanceUnit::Meter => Ok(value.pow10(-3)),
                DistanceUnit::Kilometer => Ok(value.pow10(-6)),
                _ => Err("no rules to convert it from Millimeter"),
            },
            DistanceUnit::Centimeter => match to_unit {
                DistanceUnit::Millimeter => Ok(value.pow10(1)),
                DistanceUnit::Centimeter => Ok(value),
                DistanceUnit::Meter => Ok(value.pow10(-2)),
                DistanceUnit::Kilometer => Ok(value.pow10(-5)),
                _ => Err("no rules to convert X from Centimeter"),
            },
            DistanceUnit::Meter => match to_unit {
                DistanceUnit::Millimeter => Ok(value.pow10(3)),
                DistanceUnit::Centimeter => Ok(value.pow10(2)),
                DistanceUnit::Meter => Ok(value),
                DistanceUnit::Kilometer => Ok(value.pow10(-3)),
                _ => Err("no rules to convert X from Meter"),
            },
            DistanceUnit::Kilometer => match to_unit {
                DistanceUnit::Millimeter => Ok(value.pow10(6)),
                DistanceUnit::Centimeter => Ok(value.pow10(5)),
                DistanceUnit::Meter => Ok(value.pow10(3)),
                DistanceUnit::Kilometer => Ok(value),
                _ => Err("no rules to convert X from Kilometer"),
            },
            _ => Err("can not convert metric distance to x"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DISTANCE_TOLERANCE: f64 = 0.1e-5;

    fn is_close(expected_val: f64, real_val: f64) -> bool {
        (expected_val - real_val).abs() <= DISTANCE_TOLERANCE
    }

    #[test]
    fn test_0millimeters_to_centimeters() {
        let mm = Distance::new(0.0, DistanceUnit::Millimeter);
        let res = mm.to_metric(DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_millimeter() {
        let mm = Distance::new(1.0, DistanceUnit::Millimeter);
        let res = mm.to_metric(DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_centimeter() {
        let mm = Distance::new(10.0, DistanceUnit::Millimeter);
        let res = mm.to_metric(DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_meter() {
        let mm = Distance::new(1000.0, DistanceUnit::Millimeter);
        let res = mm.to_metric(DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_kilometer() {
        let mm = Distance::new(1_000_000.0, DistanceUnit::Millimeter);
        let res = mm.to_metric(DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    // centimeters
    #[test]
    fn test_centimeters_to_millimeter() {
        let cm = Distance::new(0.1, DistanceUnit::Centimeter);
        let res = cm.to_metric(DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_centimeters_to_centimeter() {
        let mm = Distance::new(1.0, DistanceUnit::Centimeter);
        let res = mm.to_metric(DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_centimeters_to_meter() {
        let mm = Distance::new(100.0, DistanceUnit::Centimeter);
        let res = mm.to_metric(DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_centimeters_to_kilometer() {
        let mm = Distance::new(100_000.0, DistanceUnit::Centimeter);
        let res = mm.to_metric(DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    // meters
    #[test]
    fn test_meters_to_millimeter() {
        let mm = Distance::new(0.001, DistanceUnit::Meter);
        let res = mm.to_metric(DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_meters_to_centimeter() {
        let mm = Distance::new(0.01, DistanceUnit::Meter);
        let res = mm.to_metric(DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_meters_to_meter() {
        let mm = Distance::new(1.0, DistanceUnit::Meter);
        let res = mm.to_metric(DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_meters_to_kilometer() {
        let mm = Distance::new(1000.0, DistanceUnit::Meter);
        let res = mm.to_metric(DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    // kilometers to metric units
    #[test]
    fn test_kilometers_to_millimeter() {
        let mm = Distance::new(1.0e-6, DistanceUnit::Kilometer);
        let res = mm.to_metric(DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_centimeter() {
        let mm = Distance::new(1.0e-5, DistanceUnit::Kilometer);
        let res = mm.to_metric(DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_meter() {
        let mm = Distance::new(1.0e-3, DistanceUnit::Kilometer);
        let res = mm.to_metric(DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_kilometer() {
        let mm = Distance::new(1.0, DistanceUnit::Kilometer);
        let res = mm.to_metric(DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }
}
