use crate::traits::*;
use std::fmt;

pub mod astronomical_unit;
pub mod centimeter;
pub mod feet;
pub mod inch;
pub mod kilometer;
pub mod light_year;
pub mod meter;
pub mod mile;
pub mod millimeter;
pub mod parsec;

pub use astronomical_unit::AstronomicalUnit;
pub use centimeter::Centimeter;
pub use feet::Feet;
pub use inch::Inch;
pub use kilometer::Kilometer;
pub use light_year::LightYear;
pub use meter::Meter;
pub use mile::Mile;
pub use millimeter::Millimeter;
pub use parsec::Parsec;

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
        };

        write!(f, "{}", enum_label)
    }
}

struct DistanceFactory {}

impl DistanceFactory {
    pub fn build(
        value: f64,
        unit: DistanceUnit,
    ) -> Box<dyn HasConvertableUnit<Unit = DistanceUnit>> {
        match unit {
            DistanceUnit::Millimeter => Box::new(Millimeter::new(value)),
            DistanceUnit::Centimeter => Box::new(Centimeter::new(value)),
            DistanceUnit::Meter => Box::new(Meter::new(value)),
            DistanceUnit::Kilometer => Box::new(Kilometer::new(value)),
            DistanceUnit::Inch => Box::new(Inch::new(value)),
            DistanceUnit::Feet => Box::new(Feet::new(value)),
            DistanceUnit::Mile => Box::new(Mile::new(value)),
            DistanceUnit::AstronomicalUnit => Box::new(AstronomicalUnit::new(value)),
            DistanceUnit::LightYear => Box::new(LightYear::new(value)),
            DistanceUnit::Parsec => Box::new(Parsec::new(value)),
        }
    }
}

struct DistanceConverter {
    distance: Box<dyn HasConvertableUnit<Unit = DistanceUnit>>,
}

impl DistanceConverter {
    pub fn new(value: f64, unit: DistanceUnit) -> DistanceConverter {
        let distance = DistanceFactory::build(value, unit);
        DistanceConverter { distance: distance }
    }
}

impl CanConvertUnit for DistanceConverter {
    type Unit = DistanceUnit;

    fn convert<'a>(
        &self,
        to_unit: Self::Unit,
    ) -> Result<Box<dyn HasConvertableUnit<Unit = Self::Unit>>, &'a str> {
        if let Ok(val) = self.distance.as_ref().convert_scalar(&to_unit) {
            let converted_distance = DistanceFactory::build(val, to_unit);

            Ok(converted_distance)
        } else {
            Err("failed to convert the distance")
        }
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
        let conv = DistanceConverter::new(1.609_344, DistanceUnit::Kilometer);
        let res = conv.convert(DistanceUnit::Mile);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&DistanceUnit::Mile, res.unit());
    }

    #[test]
    fn test_distance_converter_from_km_to_au() {
        let conv = DistanceConverter::new(1.0e9, DistanceUnit::Kilometer);
        let res = conv.convert(DistanceUnit::AstronomicalUnit);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(6.685, res.scalar()));
        assert_eq!(&DistanceUnit::AstronomicalUnit, res.unit());
    }

    #[test]
    fn test_distance_converter_from_parsec_to_light_year() {
        let conv = DistanceConverter::new(1.0, DistanceUnit::Parsec);
        let res = conv.convert(DistanceUnit::LightYear);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(3.262, res.scalar()));
        assert_eq!(&DistanceUnit::LightYear, res.unit());
    }
}
