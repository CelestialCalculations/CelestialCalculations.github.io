use super::*;
use crate::ast_math;
use std::f64::consts::PI;

pub struct DMS {
    degree: i16,
    minute: u8,
    second: u8,
}

impl DMS {
    pub fn new(degree: i16, minute: u8, second: u8) -> DMS {
        if degree > 359 || minute > 59 || second > 59 {
            panic!("Degree must be smaller than 360, minut and second smaller than 60");
        }

        DMS {
            degree,
            minute,
            second,
        }
    }

    pub fn from_decimal(val: f64) -> DMS {
        let sign = val.signum();
        // step.2
        let dec = val.abs();
        // step.3
        let degrees = val.trunc();
        // step.4
        let min_fac = 60.0 * ast_math::frac(dec);
        let minutes = min_fac.trunc();
        // step.5
        let seconds = 60.0 * ast_math::frac(min_fac);

        DMS {
            degree: (sign * degrees) as i16,
            minute: minutes as u8,
            second: seconds as u8,
        }
    }
}

impl HasConvertableUnit for DMS {
    type Unit = AngularUnit;

    // returns decimal format
    fn scalar(&self) -> f64 {
        // step.1
        let sign = self.degree.signum();
        // step.2
        let degree = self.degree.abs() as f64;
        // step.3
        let dm: f64 = if self.second != 0 {
            (self.second as f64) / 60.0
        } else {
            0.0
        };

        // step.4
        let total_minutes: f64 = (self.minute as f64) + dm;

        // step.5
        let decimal_degs = total_minutes as f64 / 60.0;

        // step.6 & step.7
        (sign as f64) * (degree + decimal_degs)
    }

    // TODO: seems to be really to redundant
    fn unit(&self) -> &Self::Unit {
        &AngularUnit::DMS
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            AngularUnit::Radian => Ok(value * PI / 180.0),
            AngularUnit::HMS => {
                let hrs = self.degree.checked_div(15).unwrap_or(0);
                let leftover_mins = 4 * self.degree.checked_rem(15).unwrap_or(0);
                let min = self.minute.checked_div(15).unwrap_or(0) as i16;
                let leftover_secs = 4 * self.minute.checked_rem(15).unwrap_or(0);
                let secs = self.second.checked_div(15).unwrap_or(0);
                let leftover_millis = 4 * self.second.checked_rem(15).unwrap_or(0);

                let hms_decimal = (hrs as f64)
                    + (min + leftover_mins) as f64 / 60.0
                    + (secs + leftover_secs) as f64 / 3600.0
                    + leftover_millis as f64 / (3600.0 * 1000.0);
                Ok(hms_decimal)
            }
            _ => Err("not convertiable from DMS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_dms_to_decimal_scalar() {
        let dms = DMS::new(24, 13, 18);
        assert!(is_close(24.221_667, dms.scalar()));
    }

    #[test]
    fn test_dms_from_decimal() {
        let dms = DMS::from_decimal(24.221_667);

        assert_eq!(24, dms.degree);
        assert_eq!(13, dms.minute);
        assert_eq!(18, dms.second);
    }

    #[test]
    fn test_dms_to_radian() {
        let res = DMS::new(180, 0, 0).convert_scalar(&AngularUnit::Radian);
        assert!(is_close(PI, res.unwrap()));
    }

    #[test]
    fn test_dms_to_hms() {
        let res = DMS::new(50, 31, 21).convert_scalar(&AngularUnit::HMS);
        assert!(is_close(3.368_062_222, res.unwrap()), "res: {:?}", res);
    }
}
