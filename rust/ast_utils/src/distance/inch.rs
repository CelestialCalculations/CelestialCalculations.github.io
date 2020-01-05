use super::*;

pub struct Inch {
    value: f64,
}

impl Inch {
    pub fn new(value: f64) -> Inch {
        Inch { value }
    }
}

impl HasConvertableUnit for Inch {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::Inch
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            DistanceUnit::Inch => Ok(value),
            DistanceUnit::Millimeter => Ok(value * 25.4_f64),
            _ => Err("not convertable from Inch"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_1inches_to_inches() {
        let res = Inch::new(1.0).convert_scalar(&DistanceUnit::Inch);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_0inches_to_millimeters() {
        let res = Inch::new(0.0).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_inches_to_millimeter() {
        let res = Inch::new(1.0).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(25.4, res.unwrap()));
    }
}
