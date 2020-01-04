use super::*;

pub struct Kelvin {
    value: f64,
}

impl Kelvin {
    pub fn new(value: f64) -> Kelvin {
        Kelvin { value }
    }
}

impl HasConvertableUnit for Kelvin {
    type Unit = TemperatureUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &TemperatureUnit::Kelvin
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            TemperatureUnit::Kelvin => Ok(value),
            TemperatureUnit::Fahrenheit => Ok(value * (9.0 / 5.0) - 459.67_f64),
            TemperatureUnit::Celsius => Ok(value - 273.15),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0kelvin_to_kelvin() {
        let res = Kelvin::new(0.0).convert_scalar(&TemperatureUnit::Kelvin);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_kelvin_to_kelvin() {
        let res = Kelvin::new(1.0).convert_scalar(&TemperatureUnit::Kelvin);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        let res = Kelvin::new(260.0).convert_scalar(&TemperatureUnit::Fahrenheit);

        assert!(res.is_ok());
        assert!(is_close(8.329_999_9, res.unwrap()), "res was {:?}", res);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        let res = Kelvin::new(273.15).convert_scalar(&TemperatureUnit::Celsius);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()), "res was {:?}", res);
    }
}
