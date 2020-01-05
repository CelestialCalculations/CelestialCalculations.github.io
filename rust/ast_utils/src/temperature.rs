use crate::traits::*;
use std::fmt;

pub mod celsius;
pub mod fahrenheit;
pub mod kelvin;

pub use celsius::Celsius;
pub use fahrenheit::Fahrenheit;
pub use kelvin::Kelvin;

#[derive(Debug, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enum_label: &str = match self {
            TemperatureUnit::Celsius => "celsius",
            TemperatureUnit::Fahrenheit => "fahrenheit",
            TemperatureUnit::Kelvin => "kelvin",
        };

        write!(f, "{}", enum_label)
    }
}

pub struct TemperatureFactory {}

impl TemperatureFactory {
    pub fn build(
        value: f64,
        unit: TemperatureUnit,
    ) -> Box<dyn HasConvertableUnit<Unit = TemperatureUnit>> {
        match unit {
            TemperatureUnit::Celsius => Box::new(Celsius::new(value)),
            TemperatureUnit::Fahrenheit => Box::new(Fahrenheit::new(value)),
            TemperatureUnit::Kelvin => Box::new(Kelvin::new(value)),
        }
    }
}

pub struct TemperatureConverter {
    temperature: Box<dyn HasConvertableUnit<Unit = TemperatureUnit>>,
}

impl TemperatureConverter {
    pub fn new(value: f64, unit: TemperatureUnit) -> TemperatureConverter {
        let t = TemperatureFactory::build(value, unit);
        TemperatureConverter { temperature: t }
    }
}

impl CanConvertUnit for TemperatureConverter {
    type Unit = TemperatureUnit;

    fn convert<'a>(
        &self,
        to_unit: TemperatureUnit,
    ) -> Result<Box<dyn HasConvertableUnit<Unit = Self::Unit>>, &'a str> {
        if let Ok(val) = self.temperature.as_ref().convert_scalar(&to_unit) {
            let converted_val = TemperatureFactory::build(val, to_unit);

            Ok(converted_val)
        } else {
            Err("failed to convert the temperature")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_temperature_converter_from_c_to_c() {
        let conv = TemperatureConverter::new(1.0, TemperatureUnit::Celsius);
        let res = conv.convert(TemperatureUnit::Celsius);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(1.0, res.scalar()));
        assert_eq!(&TemperatureUnit::Celsius, res.unit());
    }

    #[test]
    fn test_temperature_converter_from_c_to_kelvin() {
        let conv = TemperatureConverter::new(-273.15, TemperatureUnit::Celsius);
        let res = conv.convert(TemperatureUnit::Kelvin);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(0.0, res.scalar()));
        assert_eq!(&TemperatureUnit::Kelvin, res.unit());
    }

    #[test]
    fn test_temperature_converter_from_fahrenheit_to_celsius() {
        let conv = TemperatureConverter::new(32.0, TemperatureUnit::Fahrenheit);
        let res = conv.convert(TemperatureUnit::Celsius);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(0.0, res.scalar()));
        assert_eq!(&TemperatureUnit::Celsius, res.unit());
    }

    #[test]
    fn test_temperature_converter_from_kelvin_to_celsius() {
        let conv = TemperatureConverter::new(300.0, TemperatureUnit::Kelvin);
        let res = conv.convert(TemperatureUnit::Celsius);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(is_close(26.85, res.scalar()), "res was {:?}", res.scalar());
        assert_eq!(&TemperatureUnit::Celsius, res.unit());
    }
}
