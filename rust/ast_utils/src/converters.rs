use std::fmt;

#[derive(Debug, PartialEq)]
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

pub trait HasConvertableUnit {
    fn scalar(&self) -> f64;
    fn unit(&self) -> &DistanceUnit;
    // conversion functions
    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str>;
    fn convert<'a>(&self, to_unit: DistanceUnit) -> Result<Box<dyn HasConvertableUnit>, &'a str> {
        self.convert_scalar(&to_unit)
            .and_then(|s| Ok(DistanceFactory::build(s, to_unit)))
    }
}

pub struct Millimeter {
    value: f64,
    unit: DistanceUnit,
}

impl Millimeter {
    pub fn new(value: f64) -> Millimeter {
        Millimeter {
            value: value,
            unit: DistanceUnit::Millimeter,
        }
    }
}

impl HasConvertableUnit for Millimeter {
    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &DistanceUnit {
        &self.unit
    }

    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str> {
        match to_unit {
            DistanceUnit::Millimeter => Ok(self.value),
            DistanceUnit::Centimeter => Ok(self.value.pow10(-1)),
            DistanceUnit::Meter => Ok(self.value.pow10(-3)),
            DistanceUnit::Kilometer => Ok(self.value.pow10(-6)),
            _ => Err("Millimeter misses some distances"),
        }
    }
}

// Centimeter

pub struct Centimeter {
    value: f64,
    unit: DistanceUnit,
}

impl Centimeter {
    pub fn new(value: f64) -> Centimeter {
        Centimeter {
            value: value,
            unit: DistanceUnit::Centimeter,
        }
    }
}

impl HasConvertableUnit for Centimeter {
    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &DistanceUnit {
        &self.unit
    }

    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str> {
        match to_unit {
            DistanceUnit::Millimeter => Ok(self.value.pow10(1)),
            DistanceUnit::Centimeter => Ok(self.value),
            DistanceUnit::Meter => Ok(self.value.pow10(-2)),
            DistanceUnit::Kilometer => Ok(self.value.pow10(-5)),
            _ => Err("Can not convert from Centimeter"),
        }
    }
}

// mod: Meter

pub struct Meter {
    value: f64,
    unit: DistanceUnit,
}

impl Meter {
    pub fn new(value: f64) -> Meter {
        Meter {
            value: value,
            unit: DistanceUnit::Meter,
        }
    }
}

impl HasConvertableUnit for Meter {
    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &DistanceUnit {
        &self.unit
    }

    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str> {
        let value = self.value;

        match to_unit {
            DistanceUnit::Millimeter => Ok(value.pow10(3)),
            DistanceUnit::Centimeter => Ok(value.pow10(2)),
            DistanceUnit::Meter => Ok(value),
            DistanceUnit::Kilometer => Ok(value.pow10(-3)),
            _ => Err("Can not convert from meter"),
        }
    }
}

// mod kilometer.rs
pub struct Kilometer {
    value: f64,
    unit: DistanceUnit,
}

impl Kilometer {
    pub fn new(value: f64) -> Kilometer {
        Kilometer {
            value: value,
            unit: DistanceUnit::Kilometer,
        }
    }
}

impl HasConvertableUnit for Kilometer {
    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &DistanceUnit {
        &self.unit
    }

    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str> {
        let value = self.value;

        match to_unit {
            DistanceUnit::Millimeter => Ok(value.pow10(6)),
            DistanceUnit::Centimeter => Ok(value.pow10(5)),
            DistanceUnit::Meter => Ok(value.pow10(3)),
            DistanceUnit::Kilometer => Ok(value),
            _ => Err("Can not convert from Kilometer"),
        }
    }
}

// mod: DistanceConverter
struct DistanceFactory {}

impl DistanceFactory {
    pub fn build(value: f64, unit: DistanceUnit) -> Box<dyn HasConvertableUnit> {
        match unit {
            DistanceUnit::Millimeter => Box::new(Millimeter::new(value)),
            DistanceUnit::Centimeter => Box::new(Centimeter::new(value)),
            DistanceUnit::Meter => Box::new(Meter::new(value)),
            DistanceUnit::Kilometer => Box::new(Kilometer::new(value)),
            _ => panic!("unimplemented distance unit: {}", unit),
        }
    }
}

struct DistanceConverter {
    distance: Box<dyn HasConvertableUnit>,
    error: Option<String>,
}

impl DistanceConverter {
    pub fn new(value: f64, unit: DistanceUnit) -> DistanceConverter {
        let distance = DistanceFactory::build(value, unit);
        DistanceConverter {
            distance: distance,
            error: None,
        }
    }

    pub fn convert(&self, to_unit: DistanceUnit) -> Result<Box<dyn HasConvertableUnit>, &str> {
        self.distance.as_ref().convert(to_unit)
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
        let res = Millimeter::new(0.0).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_millimeter() {
        let res = Millimeter::new(1.0).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_centimeter() {
        let res = Millimeter::new(10.0).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_meter() {
        let res = Millimeter::new(1000.0).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_kilometer() {
        let res = Millimeter::new(1_000_000.0).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    // centimeters
    #[test]
    fn test_centimeters_to_millimeter() {
        let res = Centimeter::new(0.1).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_centimeters_to_centimeter() {
        let res = Centimeter::new(1.0).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_centimeters_to_meter() {
        let res = Centimeter::new(100.0).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_centimeters_to_kilometer() {
        let res = Centimeter::new(100_000.0).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    // meters
    #[test]
    fn test_meters_to_millimeter() {
        let res = Meter::new(0.001).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_meters_to_centimeter() {
        let res = Meter::new(0.01).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_meters_to_meter() {
        let res = Meter::new(1.0).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_meters_to_kilometer() {
        let res = Meter::new(1000.0).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    // kilometers to metric units
    #[test]
    fn test_kilometers_to_millimeter() {
        let res = Kilometer::new(1.0e-6).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_centimeter() {
        let res = Kilometer::new(1.0e-5).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_meter() {
        let res = Kilometer::new(1.0e-3).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_kilometer() {
        let res = Kilometer::new(1.0).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_distance_converter_from_mm_to_mm() {
        let conv = DistanceConverter::new(1.0, DistanceUnit::Millimeter);
        let res = conv.convert(DistanceUnit::Millimeter);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Millimeter, res.unit());
    }

    #[test]
    fn test_distance_converter_from_mm_to_cm() {
        let conv = DistanceConverter::new(10.0, DistanceUnit::Millimeter);
        let res = conv.convert(DistanceUnit::Centimeter);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Centimeter, res.unit());
    }

    #[test]
    fn test_distance_converter_from_mm_to_meter() {
        let conv = DistanceConverter::new(1000.0, DistanceUnit::Millimeter);
        let res = conv.convert(DistanceUnit::Meter);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Meter, res.unit());
    }

    #[test]
    fn test_distance_converter_from_mm_to_kmeter() {
        let conv = DistanceConverter::new(1_000_000.0, DistanceUnit::Millimeter);
        let res = conv.convert(DistanceUnit::Kilometer);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Kilometer, res.unit());
    }
}
