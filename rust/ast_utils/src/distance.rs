use std::fmt;

pub mod centimeter;
pub mod feet;
pub mod inch;
pub mod kilometer;
pub mod meter;
pub mod mile;
pub mod millimeter;

pub use centimeter::Centimeter;
pub use feet::Feet;
pub use inch::Inch;
pub use kilometer::Kilometer;
pub use meter::Meter;
pub use mile::Mile;
pub use millimeter::Millimeter;

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

struct DistanceFactory {}

impl DistanceFactory {
    pub fn build(value: f64, unit: DistanceUnit) -> Box<dyn HasConvertableUnit> {
        match unit {
            DistanceUnit::Millimeter => Box::new(Millimeter::new(value)),
            DistanceUnit::Centimeter => Box::new(Centimeter::new(value)),
            DistanceUnit::Meter => Box::new(Meter::new(value)),
            DistanceUnit::Kilometer => Box::new(Kilometer::new(value)),
            DistanceUnit::Inch => Box::new(Inch::new(value)),
            DistanceUnit::Feet => Box::new(Feet::new(value)),
            DistanceUnit::Mile => Box::new(Mile::new(value)),
            _ => panic!("unimplemented distance unit: {}", unit),
        }
    }
}

struct DistanceConverter {
    distance: Box<dyn HasConvertableUnit>,
}

impl DistanceConverter {
    pub fn new(value: f64, unit: DistanceUnit) -> DistanceConverter {
        let distance = DistanceFactory::build(value, unit);
        DistanceConverter { distance: distance }
    }

    pub fn convert(&self, to_unit: DistanceUnit) -> Result<Box<dyn HasConvertableUnit>, &str> {
        self.distance.as_ref().convert(to_unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

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

    #[test]
    fn test_distance_converter_from_mm_to_inch() {
        let conv = DistanceConverter::new(25.4, DistanceUnit::Millimeter);
        let res = conv.convert(DistanceUnit::Inch);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Inch, res.unit());
    }

    #[test]
    fn test_distance_converter_from_meter_to_feet() {
        let conv = DistanceConverter::new(0.3048, DistanceUnit::Meter);
        let res = conv.convert(DistanceUnit::Feet);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Feet, res.unit());
    }

    #[test]
    fn test_distance_converter_from_kilometer_to_mile() {
        let conv = DistanceConverter::new(1.609344, DistanceUnit::Kilometer);
        let res = conv.convert(DistanceUnit::Mile);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Mile, res.unit());
    }
}
