use super::*;
use crate::ast_math;
use std::f64::consts::PI;

pub struct HMS {
    hour: i8,
    minute: u8,
    second: u8,
}

impl HMS {
    pub fn new(hour: i8, minute: u8, second: u8) -> HMS {
        if hour.abs() > 59 || minute > 59 || second > 59 {
            panic!("Hour, minute and second must be smaller than 60");
        }

        HMS {
            hour,
            minute,
            second,
        }
    }

    pub fn from_decimal(val: f64) -> HMS {
        let sign = val.signum();
        // step.2
        let val = val.abs();
        // step.3
        let hrs = val.trunc();
        // step.4
        let min_fac = 60.0 * ast_math::frac(val);
        let minutes = min_fac.trunc();
        // step.5
        let seconds = 60.0 * ast_math::frac(min_fac);

        HMS::new(
            (sign * hrs.rem_euclid(24.0)) as i8,
            minutes as u8,
            seconds as u8,
        )
    }

    pub fn to_decimal(&self) -> f64 {
        // step 1
        let sign = self.hour.signum();
        // step 2
        let hrs = self.hour.abs() as f64;
        // step 3
        let dm: f64 = if self.second != 0 {
            (self.second as f64) / 60.0
        } else {
            0.0
        };

        // step 4
        let total_minutes: f64 = (self.minute as f64) + dm;
        // step 5
        let decimal_hrs = total_minutes / 60.0;
        // step 6 and 7
        (sign as f64) * (hrs + decimal_hrs)
    }

    pub fn to_tuple(&self) -> (i8, u8, u8) {
        (self.hour, self.minute, self.second)
    }
}

impl HasConvertableUnit for HMS {
    type Unit = AngularUnit;

    // returns decimal format
    fn scalar(&self) -> f64 {
        self.to_decimal()
    }

    fn unit(&self) -> &Self::Unit {
        &AngularUnit::HMS
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            AngularUnit::HMS => Ok(value),
            AngularUnit::Radian => Ok(value * PI / 12.0),
            AngularUnit::DMS => {
                let sign: f64 = value.signum();
                let secs = (self.second as f64) * 15.0;
                let mins = (self.minute as f64) * 15.0;
                let hrs = (self.hour as f64) * 15.0;

                let dms_decimal = hrs.rem_euclid(360.0) + mins / 60.0 + secs / 3600.0;

                Ok(sign * dms_decimal)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_hms_from_decimal() {
        let res = HMS::from_decimal(12.508_334);

        assert_eq!(12, res.hour);
        assert_eq!(30, res.minute);
        assert_eq!(30, res.second);
    }

    #[test]
    fn test_hms_to_hms() {
        let res = HMS::new(12, 30, 30).convert_scalar(&AngularUnit::HMS);

        assert!(is_close(12.508_334, res.unwrap()), "res was {:?}", res);
    }

    #[test]
    fn test_hms_to_radians() {
        let res = HMS::new(12, 0, 0).convert_scalar(&AngularUnit::Radian);

        assert!(is_close(PI, res.unwrap()), "res was {:?}", res);
    }

    #[test]
    fn test_hms_to_dms_decimal() {
        let res = HMS::new(12, 30, 30).convert_scalar(&AngularUnit::DMS);

        assert!(is_close(187.625, res.unwrap()), "res was {:?}", res);
    }
}
