use super::*;

pub struct Mile {
    value: f64,
}

impl Mile {
    pub fn new(value: f64) -> Mile {
        Mile { value }
    }
}

impl HasConvertableUnit for Mile {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::Mile
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        if value == 0.0 {
            return Ok(0.0);
        }

        match to_unit {
            DistanceUnit::Mile => Ok(value),
            DistanceUnit::Kilometer => Ok(value * 1.609_344),
            DistanceUnit::LightYear => Ok(value * 5.87e-12),
            DistanceUnit::AstronomicalUnit => Ok(value * 9.29e-7),
            _ => Err("not convertable from Mile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0miles_to_mile() {
        let res = Mile::new(0.0).convert_scalar(&DistanceUnit::Mile);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_miles_to_kilometer() {
        let res = Mile::new(0.621_371_19).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_miles_to_light_year() {
        let res = Mile::new(1.0e12).convert_scalar(&DistanceUnit::LightYear);

        assert!(res.is_ok());
        assert!(is_close(5.87, res.unwrap()));
    }

    #[test]
    fn test_miles_to_au() {
        let res = Mile::new(1.0e7).convert_scalar(&DistanceUnit::AstronomicalUnit);

        assert!(res.is_ok());
        assert!(is_close(9.29, res.unwrap()));
    }
}
