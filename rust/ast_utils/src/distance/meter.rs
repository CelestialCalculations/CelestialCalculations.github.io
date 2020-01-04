use super::*;

pub struct Meter {
    value: f64,
}

impl Meter {
    pub fn new(value: f64) -> Meter {
        Meter { value }
    }
}

impl HasConvertableUnit for Meter {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::Meter
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        if value == 0.0 {
            return Ok(0.0);
        }

        match to_unit {
            DistanceUnit::Millimeter => Ok(value.pow10(3)),
            DistanceUnit::Centimeter => Ok(value.pow10(2)),
            DistanceUnit::Meter => Ok(value),
            DistanceUnit::Kilometer => Ok(value.pow10(-3)),
            DistanceUnit::Feet => Ok(value / 0.3048),
            _ => Err("not convertable from Meter"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0meters_to_meters() {
        let res = Meter::new(0.0).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

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

    #[test]
    fn test_meters_to_feet() {
        let res = Meter::new(0.3048).convert_scalar(&DistanceUnit::Feet);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }
}
