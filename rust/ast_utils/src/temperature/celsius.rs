use super::*;

pub struct Celsius {
    value: f64,
}

impl Celsius {
    pub fn new(value: f64) -> Celsius {
        Celsius { value }
    }
}

impl HasConvertableUnit for Celsius {
    type Unit = TemperatureUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &TemperatureUnit::Celsius
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        match to_unit {
            TemperatureUnit::Celsius => Ok(value),
            TemperatureUnit::Fahrenheit => Ok(32.0_f64 + 9.0 / 5.0 * value),
            TemperatureUnit::Kelvin => Ok(273.15_f64 + value),
            _ => Err("not convertable from Celsius"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0celsius_to_celsius() {
        let res = Celsius::new(0.0).convert_scalar(&TemperatureUnit::Celsius);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_celsius_to_celsius() {
        let res = Celsius::new(1.0).convert_scalar(&TemperatureUnit::Celsius);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        let res = Celsius::new(37.777_777_777_8).convert_scalar(&TemperatureUnit::Fahrenheit);

        assert!(res.is_ok());
        assert!(is_close(100.0, res.unwrap()), "res was {:?}", res);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let res = Celsius::new(0.0).convert_scalar(&TemperatureUnit::Kelvin);

        assert!(res.is_ok());
        assert!(is_close(273.15, res.unwrap()), "res was {:?}", res);
    }
}
