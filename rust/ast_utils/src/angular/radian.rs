use super::*;
use std::f64::consts::PI;

pub struct Radian {
    value: f64,
}

impl Radian {
    pub fn new(value: f64) -> Radian {
        Radian { value }
    }
}

impl HasConvertableUnit for Radian {
    type Unit = AngularUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &AngularUnit::Radian
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            AngularUnit::Radian => Ok(value),
            AngularUnit::DMS => Ok((value * 180.0 / PI).rem_euclid(360.0)),
            AngularUnit::HMS => Ok((value * 12.0 / PI).rem_euclid(24.0)),
            _ => Err("can not convert from Radian"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0radian_to_radian() {
        let res = Radian::new(0.0).convert_scalar(&AngularUnit::Radian);
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_1radian_to_degree() {
        let res = Radian::new(1.0).convert_scalar(&AngularUnit::DMS);

        assert!(is_close(57.295_779_5, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_2pi_radians_are_wrapped_to_0_degrees() {
        let res = Radian::new(2.0 * PI).convert_scalar(&AngularUnit::DMS);

        assert!(is_close(0.0, res.unwrap()), "res was: {:?}", res);
    }

    #[test]
    fn test_radians_to_hours() {
        let res = Radian::new(PI).convert_scalar(&AngularUnit::HMS);

        assert!(is_close(12.0, res.unwrap()), "res was: {:?}", res);
    }
}
