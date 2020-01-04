use super::*;

pub struct LightYear {
    value: f64,
}

impl LightYear {
    pub fn new(value: f64) -> LightYear {
        LightYear { value }
    }
}

impl HasConvertableUnit for LightYear {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::LightYear
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            DistanceUnit::LightYear => Ok(value),
            DistanceUnit::Mile => Ok(value * 5.879e12),
            DistanceUnit::Kilometer => Ok(value * 9.461e12),
            DistanceUnit::Parsec => Ok(value * 0.3066),
            DistanceUnit::AstronomicalUnit => Ok(value * 63_241.0),
            _ => Err("not convertable from LightYear"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0light_year_to_light_year() {
        let res = LightYear::new(0.0).convert_scalar(&DistanceUnit::LightYear);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_light_year_to_mile() {
        let res = LightYear::new(1.0e-12).convert_scalar(&DistanceUnit::Mile);

        assert!(res.is_ok());
        assert!(is_close(5.879, res.unwrap()));
    }

    #[test]
    fn test_light_year_to_kilometer() {
        let res = LightYear::new(1.0e-12).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(9.461, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_light_year_to_parsec() {
        let res = LightYear::new(10.0).convert_scalar(&DistanceUnit::Parsec);

        assert!(res.is_ok());
        assert!(is_close(3.066, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_light_year_to_au() {
        let res = LightYear::new(1.0).convert_scalar(&DistanceUnit::AstronomicalUnit);

        assert!(res.is_ok());
        assert!(is_close(63_241.0, res.unwrap()), "res was: {:?}", res);
    }
}
