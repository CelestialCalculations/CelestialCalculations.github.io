#[derive(Debug)]
pub enum MetricUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
}

trait ToMetricDistance {
    fn to_metric(&self, value: f64, to_unit: MetricUnit) -> Result<f64, &str>;
    fn from_metric(&self, value: f64, from_unit: MetricUnit) -> Result<f64, &str>;
}

impl ToMetricDistance for MetricUnit {
    // TODO: metrics converters are duplicated,
    // add function to scale values in 10th power power - f64.pow10(-3) = f64 / 1000.0
    // it means we could make the match-clause to return the scale
    // and actual calculation happens after that;
    // TODO: is Results monad over-kill here?

    fn to_metric(&self, value: f64, to_unit: MetricUnit) -> Result<f64, &str> {
        match self {
            MetricUnit::Millimeter => match to_unit {
                MetricUnit::Millimeter => Ok(value),
                MetricUnit::Centimeter => Ok(value / 10.0),
                MetricUnit::Meter => Ok(value / 1000.0),
                MetricUnit::Kilometer => Ok(value / 1_000_000.0),
            },
            MetricUnit::Centimeter => match to_unit {
                MetricUnit::Millimeter => Ok(value * 10.0),
                MetricUnit::Centimeter => Ok(value),
                MetricUnit::Meter => Ok(value / 100.0),
                MetricUnit::Kilometer => Ok(value / 100_000.0),
            },
            MetricUnit::Meter => match to_unit {
                MetricUnit::Millimeter => Ok(value * 1000.0),
                MetricUnit::Centimeter => Ok(value * 100.0),
                MetricUnit::Meter => Ok(value),
                MetricUnit::Kilometer => Ok(value / 1000.0),
            },
            MetricUnit::Kilometer => match to_unit {
                MetricUnit::Millimeter => Ok(value / 1_000_000.0),
                MetricUnit::Centimeter => Ok(value / 100_000.0),
                MetricUnit::Meter => Ok(value / 1000.0),
                MetricUnit::Kilometer => Ok(value),
            },
        }
    }

    fn from_metric(&self, value: f64, from_unit: MetricUnit) -> Result<f64, &str> {
        todo!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO add more tests
    const DISTANCE_TOLERANCE: f64 = 0.1e-5;

    fn is_small_error(expected_val: f64, real_val: f64) -> bool {
        (expected_val - real_val).abs() <= DISTANCE_TOLERANCE
    }

    #[test]
    fn test_10millimeters_to_centimeters() {
        let mm = MetricUnit::Millimeter;
        let res = mm.to_metric(10.0, MetricUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_small_error(1.0, res.unwrap()));
    }
}
