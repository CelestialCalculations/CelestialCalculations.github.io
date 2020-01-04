use super::*;

pub struct Parsec {
    value: f64, // be carefurl converting to KM or miles
}

impl Parsec {
    pub fn new(value: f64) -> Parsec {
        Parsec { value }
    }
}

impl HasConvertableUnit for Parsec {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::Parsec
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            DistanceUnit::Parsec => Ok(value),
            DistanceUnit::Kilometer => Ok(value * 3.086e13),
            DistanceUnit::Mile => Ok(value * 19.17e12),
            DistanceUnit::AstronomicalUnit => Ok(value * 2.063e5),
            DistanceUnit::LightYear => Ok(value * 3.261_564),
            _ => Err("not convertable from Parsec"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0parsec_to_parsec() {
        let res = Parsec::new(0.0).convert_scalar(&DistanceUnit::Parsec);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_parsec_to_kilometer() {
        let res = Parsec::new(1.0e-13).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(3.086, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_parsec_to_mile() {
        let res = Parsec::new(1.0e-12).convert_scalar(&DistanceUnit::Mile);

        assert!(res.is_ok());
        assert!(is_close(19.17, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_parsec_to_au() {
        let res = Parsec::new(1.0).convert_scalar(&DistanceUnit::AstronomicalUnit);

        assert!(res.is_ok());
        assert!(is_close(206_300.0, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_parsec_to_lightyear() {
        let res = Parsec::new(1.0).convert_scalar(&DistanceUnit::LightYear);

        assert!(res.is_ok());
        assert!(is_close(3.261_564, res.unwrap()), "res was: {:?}", res);
    }
}
