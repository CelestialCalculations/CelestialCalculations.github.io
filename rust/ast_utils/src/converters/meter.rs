use super::*;

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

mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

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
}
