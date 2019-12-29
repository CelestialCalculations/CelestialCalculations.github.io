use super::*;

pub struct Mile {
    value: f64,
    unit: DistanceUnit,
}

impl Mile {
    pub fn new(value: f64) -> Mile {
        Mile {
            value,
            unit: DistanceUnit::Mile,
        }
    }
}

impl HasConvertableUnit for Mile {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &DistanceUnit {
        &self.unit
    }

    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str> {
        let value = self.scalar();

        if value == 0.0 {
            return Ok(0.0);
        }

        match to_unit {
            DistanceUnit::Mile => Ok(value),
            DistanceUnit::Kilometer => Ok(value * 1.609_344),
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
}
