use super::*;

pub struct Feet {
    value: f64,
    unit: DistanceUnit,
}

impl Feet {
    pub fn new(value: f64) -> Feet {
        Feet {
            value: value,
            unit: DistanceUnit::Feet,
        }
    }
}

impl HasConvertableUnit for Feet {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &DistanceUnit {
        &self.unit
    }

    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            DistanceUnit::Feet => Ok(value),
            DistanceUnit::Meter => Ok(value * 0.3048),
            _ => Err("not convertable from Feet"),
        }
    }
}

mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0feet_to_feet() {
        let res = Feet::new(0.0).convert_scalar(&DistanceUnit::Feet);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_feet_to_meters() {
        let res = Feet::new(3.2808399).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }
}
