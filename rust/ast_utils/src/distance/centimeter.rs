use super::*;

pub struct Centimeter {
    value: f64,
}

impl Centimeter {
    pub fn new(value: f64) -> Centimeter {
        Centimeter { value }
    }
}

impl HasConvertableUnit for Centimeter {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::Centimeter
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        match to_unit {
            DistanceUnit::Millimeter => Ok(self.value.pow10(1)),
            DistanceUnit::Centimeter => Ok(self.value),
            DistanceUnit::Meter => Ok(self.value.pow10(-2)),
            DistanceUnit::Kilometer => Ok(self.value.pow10(-5)),
            _ => Err("Can not convert from Centimeter"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

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
}
