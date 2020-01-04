use super::*;

pub struct AstronomicalUnit {
    value: f64, // it wouldnt work for long distances;
}

impl AstronomicalUnit {
    pub fn new(value: f64) -> AstronomicalUnit {
        AstronomicalUnit { value }
    }
}

impl HasConvertableUnit for AstronomicalUnit {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::AstronomicalUnit
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            DistanceUnit::AstronomicalUnit => Ok(value),
            DistanceUnit::Mile => Ok(value * 9.29e7),
            DistanceUnit::Kilometer => Ok(value * 1.496e8),
            DistanceUnit::Parsec => Ok(value * 4.848e-6),
            DistanceUnit::LightYear => Ok(value * 1.581e-5),
            _ => Err("not convertable from Astronomical Unit"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0au_to_au() {
        let res = AstronomicalUnit::new(0.0).convert_scalar(&DistanceUnit::AstronomicalUnit);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_au_to_kilometers() {
        let res = AstronomicalUnit::new(1.0e-8).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.496, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_au_to_miles() {
        let res = AstronomicalUnit::new(1.0e-7).convert_scalar(&DistanceUnit::Mile);

        assert!(res.is_ok());
        assert!(is_close(9.29, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_au_to_parsec() {
        let res = AstronomicalUnit::new(1.0e6).convert_scalar(&DistanceUnit::Parsec);

        assert!(res.is_ok());
        assert!(is_close(4.848, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_au_to_lightyear() {
        let res = AstronomicalUnit::new(1.0e5).convert_scalar(&DistanceUnit::LightYear);

        assert!(res.is_ok());
        assert!(is_close(1.581, res.unwrap()), "res was: {:?}", res);
    }
}
