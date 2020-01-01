use crate::traits::*;
use std::fmt;

pub mod dms;
pub mod hms;
pub mod radian;

use dms::DMS;
use hms::HMS;
use radian::Radian;

#[derive(Debug, PartialEq)]
pub enum AngularUnit {
    DMS, // Degree, Minut, Second
    HMS, // Hour Minut, Second
    Radian,
}

impl fmt::Display for AngularUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enum_label: &str = match self {
            AngularUnit::DMS => "dms",
            AngularUnit::HMS => "hms",
            AngularUnit::Radian => "rad",
        };

        write!(f, "{}", enum_label)
    }
}

struct AngularFactory {}

impl AngularFactory {
    pub fn build(value: f64, unit: AngularUnit) -> Box<dyn HasConvertableUnit<Unit = AngularUnit>> {
        match unit {
            AngularUnit::DMS => Box::new(DMS::from_decimal(value)),
            AngularUnit::HMS => Box::new(HMS::from_decimal(value)),
            AngularUnit::Radian => Box::new(Radian::new(value)),
        }
    }
}

struct AngularConverter {
    angle: Box<dyn HasConvertableUnit<Unit = AngularUnit>>,
}

impl AngularConverter {
    pub fn new(value: f64, unit: AngularUnit) -> AngularConverter {
        let angle = AngularFactory::build(value, unit);

        AngularConverter { angle: angle }
    }
}

impl CanConvertUnit for AngularConverter {
    type Unit = AngularUnit;

    fn convert<'a>(
        &self,
        to_unit: AngularUnit,
    ) -> Result<Box<dyn HasConvertableUnit<Unit = Self::Unit>>, &'a str> {
        if let Ok(val) = self.angle.as_ref().convert_scalar(&to_unit) {
            let converted_angle = AngularFactory::build(val, to_unit);

            Ok(converted_angle)
        } else {
            Err("Failed to convert angle")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_angular_converter_from_dms_to_radian() {
        let conv = AngularConverter::new(187.625, AngularUnit::DMS);
        let res = conv.convert(AngularUnit::Radian);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(3.274_674, res.scalar()));
        assert_eq!(&AngularUnit::Radian, res.unit());
    }

    #[test]
    fn test_angular_converter_from_hms_to_radian() {
        let conv = AngularConverter::new(12.508_333, AngularUnit::HMS);
        let res = conv.convert(AngularUnit::Radian);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(
            is_close(3.274_601, res.scalar()),
            "res was: {}",
            res.scalar()
        );
        assert_eq!(&AngularUnit::Radian, res.unit());
    }

    #[test]
    fn test_angular_converter_from_radian_to_hms() {
        let conv = AngularConverter::new(3.274_601, AngularUnit::Radian);
        let res = conv.convert(AngularUnit::HMS);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(
            is_close(12.507_777, res.scalar()),
            "res was: {}",
            res.scalar()
        );
        assert_eq!(&AngularUnit::HMS, res.unit());
    }
}
