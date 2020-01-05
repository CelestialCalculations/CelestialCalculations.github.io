use super::*;

pub struct Fahrenheit {
    value: f64,
}

impl Fahrenheit {
    pub fn new(value: f64) -> Fahrenheit {
        Fahrenheit { value }
    }
}

impl HasConvertableUnit for Fahrenheit {
    type Unit = TemperatureUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &TemperatureUnit::Fahrenheit
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            TemperatureUnit::Fahrenheit => Ok(value),
            TemperatureUnit::Celsius => Ok(5.0 / 9.0 * (value - 32_f64)),
            TemperatureUnit::Kelvin => Ok((459.67_f64 + value) * 5.0 / 9.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0fahrenheit_to_fahrenheit() {
        let res = Fahrenheit::new(0.0).convert_scalar(&TemperatureUnit::Fahrenheit);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_fahrenheit_to_fahrenheit() {
        let res = Fahrenheit::new(1.0).convert_scalar(&TemperatureUnit::Fahrenheit);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_fahrentheit_to_celsius() {
        let res = Fahrenheit::new(100.0).convert_scalar(&TemperatureUnit::Celsius);

        assert!(res.is_ok());
        assert!(
            is_close(37.777_777_777_8, res.unwrap()),
            "res was {:?}",
            res
        );
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        let res = Fahrenheit::new(0.0).convert_scalar(&TemperatureUnit::Kelvin);

        assert!(res.is_ok());
        assert!(
            is_close(255.372_222_222_222, res.unwrap()),
            "res was {:?}",
            res
        );
    }
}
